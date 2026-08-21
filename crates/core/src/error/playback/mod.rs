//! What can go wrong when asking a backend to play music.

use std::time::Duration;

/// Result alias for the playback layer.
pub type PlaybackResult<T> = Result<T, PlaybackError>;

/// A classified playback failure.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PlaybackError {
    /// No device is available to receive commands.
    #[error("No active device — press d to choose one")]
    NoActiveDevice,

    /// Playback control requires a Premium subscription.
    #[error("Spotify Premium is required to control playback")]
    PremiumRequired,

    /// The device or account refused this specific action.
    #[error("Spotify would not allow that right now")]
    Forbidden,

    /// Too many requests; the worker backs off for this long.
    #[error("Slowing down — Spotify is rate limiting requests")]
    RateLimited(
        /// How long to wait before retrying.
        Duration,
    ),

    /// The session could not be renewed.
    #[error("Session expired — restart termify to sign in again")]
    AuthExpired,

    /// The account is not on the app's allowlist.
    #[error(
        "This account is not allowed to use your Spotify app.\n\
         Add it under Users and Access in the developer dashboard."
    )]
    NotAllowlisted,

    /// The network is unreachable, or the request timed out.
    #[error("Cannot reach Spotify — check your connection")]
    Unreachable,

    /// The backend replied with something unexpected.
    #[error("Spotify returned an unexpected response ({status})")]
    Unexpected {
        /// HTTP status code.
        status: u16,
    },

    /// The reply could not be understood.
    #[error("Could not read Spotify's reply")]
    Malformed,

    /// A URI we hold could not be used for playback.
    #[error("That item cannot be played from termify")]
    Unplayable,
}

impl PlaybackError {
    /// Whether the worker should retry rather than surface this.
    #[must_use]
    pub const fn is_transient(&self) -> bool {
        matches!(self, Self::RateLimited(_) | Self::Unreachable)
    }

    /// Whether this is an ordinary state rather than a fault.
    #[must_use]
    pub const fn is_expected_state(&self) -> bool {
        matches!(self, Self::NoActiveDevice)
    }
}
