//! The browser hand-off: announce, open, wait, exchange.

use rspotify::AuthCodePkceSpotify;
use rspotify::prelude::OAuthClient;

use crate::constant::CONSENT_TIMEOUT;

use super::AuthError;
use super::callback::wait_for_code;

/// Runs the browser half of the PKCE flow and exchanges the code for a token.
pub(crate) async fn consent<F>(
    client: &mut AuthCodePkceSpotify,
    port: u16,
    announce: F,
) -> Result<(), AuthError>
where
    F: FnOnce(&str),
{
    let url = client
        .get_authorize_url(None)
        .map_err(|source| AuthError::Exchange {
            source: Box::new(source),
        })?;

    announce(&url);
    if let Err(error) = open::that_detached(&url) {
        tracing::warn!(%error, "could not open a browser; the URL was printed instead");
    }

    let expected_state = client.oauth.state.clone();
    let code = tokio::time::timeout(CONSENT_TIMEOUT, wait_for_code(port, &expected_state))
        .await
        .map_err(|_| AuthError::TimedOut)??;

    client
        .request_token(&code)
        .await
        .map_err(|source| AuthError::Exchange {
            source: Box::new(source),
        })
}
