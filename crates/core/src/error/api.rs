use std::time::Duration;

use rspotify::{ClientError, http::HttpError};

use crate::{
    constant::{DEFAULT_BACKOFF, MAX_BACKOFF},
    error::PlaybackError,
};

/// Which group of endpoint a failure came from.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endpoint {
    /// `/me/player/*`
    Player,
    /// `/me/player/devices`
    Devices,
    /// `/me`
    Profile,
    /// Catalogue and library reads: `/search`, `/me/playlists`, `/albums/…`.
    Browse,
}

impl Endpoint {
    /// Classifies an rspotify error against the endpoint it came from.
    #[must_use]
    pub fn classify(self, error: &ClientError) -> PlaybackError {
        match error {
            ClientError::Http(http) => self.from_http(http),
            ClientError::ParseJson(_) | ClientError::ParseUrl(_) | ClientError::Model(_) => {
                tracing::warn!(%error, "could not parse Spotify response");
                PlaybackError::Malformed
            }
            ClientError::InvalidToken => PlaybackError::AuthExpired,
            ClientError::Io(_) => PlaybackError::Unreachable,
            other => {
                tracing::warn!(error = %other, "unclassified Spotify client error");
                PlaybackError::Unexpected { status: 0 }
            }
        }
    }
    fn from_http(self, error: &HttpError) -> PlaybackError {
        let HttpError::StatusCode(response) = error else {
            // The request never completed: DNS, TLS, connection, or timeout.
            return PlaybackError::Unreachable;
        };

        let status = response.status();

        match status.as_u16() {
            401 => PlaybackError::AuthExpired,
            403 => match self {
                // A `403` on a player endpoint is nearly always the Premium
                // requirement; on anything else it is an access problem.
                Endpoint::Player => PlaybackError::PremiumRequired,
                Endpoint::Profile => PlaybackError::NotAllowlisted,
                Endpoint::Devices | Endpoint::Browse => PlaybackError::Forbidden,
            },
            404 => match self {
                Endpoint::Player | Endpoint::Devices => PlaybackError::NoActiveDevice,
                Endpoint::Profile | Endpoint::Browse => PlaybackError::Unexpected { status: 404 },
            },
            429 => {
                // Read `Retry-After` by name rather than by constant, so `reqwest`
                // stays an indirect dependency.
                let backoff = response
                    .headers()
                    .get("retry-after")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|value| value.trim().parse::<u64>().ok())
                    .map_or(DEFAULT_BACKOFF, |seconds| {
                        Duration::from_secs(seconds).clamp(Duration::from_secs(1), MAX_BACKOFF)
                    });
                PlaybackError::RateLimited(backoff)
            }
            other => PlaybackError::Unexpected { status: other },
        }
    }
}
