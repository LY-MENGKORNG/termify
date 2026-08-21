use std::time::Duration;

/// Fallback back-off when Spotify omits `Retry-After`.
pub const DEFAULT_BACKOFF: Duration = Duration::from_secs(5);

/// Upper bound on how long a rate-limit back-off may last.
pub const MAX_BACKOFF: Duration = Duration::from_secs(60);

/// Spotify's "keymaster" client id, which librespot presents at login5.
pub const CLIENT_ID: &str = "65b708073fc0480ea92a077233ca87bd";
