//! Transient messages shown as toasts.

use std::time::{Duration, Instant};

/// How long a message stays on screen before expiring.
const DEFAULT_TTL: Duration = Duration::from_secs(4);

/// Errors linger: they usually carry an instruction the user must act on.
const ERROR_TTL: Duration = Duration::from_secs(8);

/// Severity, which selects the accent color and the lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    /// Neutral information.
    Info,
    /// Something worked.
    Success,
    /// Something needs attention; the application carries on.
    Warning,
    /// Something failed.
    Error,
}

/// A message queued for display.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Severity.
    pub level: Level,
    /// Text shown to the user. Should name a next step where one exists.
    pub message: String,
    /// When the notification was raised.
    pub raised_at: Instant,
    /// How long it remains visible.
    pub ttl: Duration,
}

impl Notification {
    /// Neutral information.
    #[must_use]
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Level::Info, message, DEFAULT_TTL)
    }

    /// A confirmation.
    #[must_use]
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(Level::Success, message, DEFAULT_TTL)
    }

    /// A warning.
    #[must_use]
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Level::Warning, message, ERROR_TTL)
    }

    /// A failure.
    #[must_use]
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Level::Error, message, ERROR_TTL)
    }

    fn new(level: Level, message: impl Into<String>, ttl: Duration) -> Self {
        Self {
            level,
            message: message.into(),
            raised_at: Instant::now(),
            ttl,
        }
    }

    /// Whether the notification should be dropped as of `now`.
    #[must_use]
    pub fn is_expired(&self, now: Instant) -> bool {
        now.duration_since(self.raised_at) >= self.ttl
    }
}
