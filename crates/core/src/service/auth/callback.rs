//! The one HTTP server termify runs.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::constant::{CALLBACK_PAGE, MAX_REQUEST_LINE};

use super::AuthError;

/// Waits for Spotify to redirect the browser back to us.
pub(super) async fn wait_for_code(port: u16, expected_state: &str) -> Result<String, AuthError> {
    let listener = bind_loopback(port).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        // Scoped so the borrow ends before the response is written. The read is
        // capped: a malformed request must not be able to allocate without limit.
        let request_line = {
            let mut reader = BufReader::new((&mut stream).take(MAX_REQUEST_LINE));
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            line
        };

        if request_line.trim().is_empty() {
            continue;
        }

        let Some(target) = request_target(&request_line) else {
            continue;
        };

        let outcome = parse_callback(target, expected_state);

        if matches!(outcome, CallbackOutcome::NotTheCallback) {
            // Anything that is not the callback is left unanswered so the
            // browser goes on to make the request we are actually waiting for.
            continue;
        }

        let _ = stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\
                     Content-Length: {}\r\nConnection: close\r\n\r\n{CALLBACK_PAGE}",
                    CALLBACK_PAGE.len()
                )
                .as_bytes(),
            )
            .await;
        let _ = stream.flush().await;

        return match outcome {
            CallbackOutcome::Code(code) => Ok(code),
            CallbackOutcome::Declined(reason) => Err(AuthError::Declined { reason }),
            CallbackOutcome::StateMismatch => Err(AuthError::StateMismatch),
            CallbackOutcome::NotTheCallback => Err(AuthError::EmptyCallback),
        };
    }
}

/// Binds the loopback interface, preferring IPv4 and falling back to IPv6.
async fn bind_loopback(port: u16) -> Result<TcpListener, AuthError> {
    let v4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);

    match TcpListener::bind(v4).await {
        Ok(listener) => Ok(listener),
        Err(v4_error) => {
            let v6 = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), port);
            TcpListener::bind(v6).await.map_err(|_| AuthError::Listen {
                port,
                source: v4_error,
            })
        }
    }
}

/// Extracts the request target from a request line such as `GET /cb?x=1 HTTP/1.1`.
fn request_target(request_line: &str) -> Option<&str> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    if !method.eq_ignore_ascii_case("GET") {
        return None;
    }
    parts.next()
}

/// What a callback request turned out to be.
#[derive(Debug, PartialEq, Eq)]
enum CallbackOutcome {
    /// An authorization code, with a matching state.
    Code(String),
    /// Spotify reported a failure.
    Declined(String),
    /// A code arrived, but the state did not match.
    StateMismatch,
    /// Some other request, e.g. `/favicon.ico`.
    NotTheCallback,
}

/// Classifies the query string of a callback request.
fn parse_callback(target: &str, expected_state: &str) -> CallbackOutcome {
    let Some((_, query)) = target.split_once('?') else {
        return CallbackOutcome::NotTheCallback;
    };

    let mut code = None;
    let mut state = None;
    let mut error = None;
    let mut description = None;

    for (key, value) in query_pairs(query) {
        match key.as_str() {
            "code" => code = Some(value),
            "state" => state = Some(value),
            "error" => error = Some(value),
            "error_description" => description = Some(value),
            _ => {}
        }
    }

    if let Some(error) = error {
        let reason = description.unwrap_or(error);
        return CallbackOutcome::Declined(reason);
    }

    let Some(code) = code else {
        return CallbackOutcome::NotTheCallback;
    };

    // Compare against the state we generated: without this, any page the user
    // visits could complete a sign-in on their behalf.
    if state.as_deref() != Some(expected_state) {
        return CallbackOutcome::StateMismatch;
    }

    CallbackOutcome::Code(code)
}

/// Splits and decodes an `application/x-www-form-urlencoded` query string.
fn query_pairs(query: &str) -> impl Iterator<Item = (String, String)> {
    query.split('&').filter_map(|pair| {
        if pair.is_empty() {
            return None;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        Some((percent_decode(key), percent_decode(value)))
    })
}

/// Decodes `%XX` escapes and `+` as space, leaving invalid escapes verbatim.
fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes.get(index) {
            Some(b'+') => {
                out.push(b' ');
                index += 1;
            }
            Some(b'%') => {
                let decoded = input
                    .get(index + 1..index + 3)
                    .and_then(|hex| u8::from_str_radix(hex, 16).ok());

                if let Some(byte) = decoded {
                    out.push(byte);
                    index += 3;
                } else {
                    // Not a valid escape: keep the `%` verbatim rather than
                    // silently dropping a character the user typed.
                    out.push(b'%');
                    index += 1;
                }
            }
            Some(byte) => {
                out.push(*byte);
                index += 1;
            }
            None => break,
        }
    }

    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_code_when_state_matches() {
        let outcome = parse_callback("/callback?code=abc123&state=xyz", "xyz");
        assert_eq!(outcome, CallbackOutcome::Code("abc123".to_owned()));
    }

    #[test]
    fn rejects_mismatched_state() {
        let outcome = parse_callback("/callback?code=abc123&state=other", "xyz");
        assert_eq!(outcome, CallbackOutcome::StateMismatch);
    }

    #[test]
    fn reports_the_description_when_declined() {
        let outcome = parse_callback(
            "/callback?error=access_denied&error_description=User+said+no",
            "xyz",
        );
        assert_eq!(
            outcome,
            CallbackOutcome::Declined("User said no".to_owned())
        );
    }

    #[test]
    fn ignores_unrelated_requests() {
        assert_eq!(
            parse_callback("/favicon.ico", "xyz"),
            CallbackOutcome::NotTheCallback
        );
        assert_eq!(
            parse_callback("/callback?state=xyz", "xyz"),
            CallbackOutcome::NotTheCallback
        );
    }

    #[test]
    fn reads_the_target_from_a_get_line() {
        assert_eq!(
            request_target("GET /callback?code=1 HTTP/1.1\r\n"),
            Some("/callback?code=1")
        );
        assert_eq!(request_target("POST /callback HTTP/1.1\r\n"), None);
    }

    #[test]
    fn decodes_percent_escapes() {
        assert_eq!(percent_decode("a%20b+c"), "a b c");
        // An invalid escape is preserved rather than swallowed.
        assert_eq!(percent_decode("100%"), "100%");
        assert_eq!(percent_decode("%zz"), "%zz");
    }
}
