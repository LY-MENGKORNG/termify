use std::io;

use thiserror::Error;

/// Authentication failed.
#[derive(Debug, Error)]
pub enum AuthError {
    /// The configured redirect URI has no usable port.
    #[error("`redirect_uri` needs an explicit port so termify can listen for the callback")]
    NoCallbackPort,

    /// The callback port is already taken.
    #[error(
        "could not listen on 127.0.0.1:{port} — {source}\n\n\
         Another program is using that port. Change `redirect_uri` in your \
         configuration (and in the Spotify dashboard) to a free one."
    )]
    Listen {
        /// Port we tried to bind.
        port: u16,
        /// Underlying I/O failure.
        source: io::Error,
    },

    /// The user did not return from the browser in time.
    #[error("timed out waiting for the browser; run termify again to retry")]
    TimedOut,

    /// The user declined, or Spotify rejected the request.
    #[error("Spotify declined the sign-in: {reason}")]
    Declined {
        /// Reason reported in the callback query string.
        reason: String,
    },

    /// The callback did not carry the state we generated.
    #[error("the sign-in response did not match this request; please try again")]
    StateMismatch,

    /// The token exchange failed.
    #[error("could not exchange the authorization code: {source}")]
    Exchange {
        /// Underlying rspotify failure.
        source: Box<rspotify::ClientError>,
    },

    /// The profile request failed, so the session is unusable.
    #[error(
        "signed in, but Spotify refused to return your profile: {source}\n\n\
         If your app is in Development Mode, add this account under \
         Users and Access in the developer dashboard."
    )]
    Profile {
        /// Underlying rspotify failure.
        source: Box<rspotify::ClientError>,
    },

    /// The connection carried no readable request.
    #[error("the browser callback was empty; please try again")]
    EmptyCallback,

    /// Talking to the callback socket failed.
    #[error("failed while reading the browser callback: {0}")]
    Io(#[from] io::Error),

    /// The exchange succeeded but produced nothing usable.
    #[error("Spotify returned an empty access token; please try again")]
    EmptyToken,

    /// The cached session has no refresh token, so it cannot be renewed.
    #[error("this session cannot be renewed; run `termify --logout` and sign in again")]
    NoRefreshToken,

    /// Spotify refused to renew the token.
    #[error("could not renew the Spotify session: {source}")]
    Refresh {
        /// Underlying rspotify failure.
        source: Box<rspotify::ClientError>,
    },
}
