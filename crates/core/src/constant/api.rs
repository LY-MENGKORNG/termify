use std::time::Duration;

/// Fallback back-off when Spotify omits `Retry-After`.
pub const DEFAULT_BACKOFF: Duration = Duration::from_secs(5);

/// Upper bound on how long a rate-limit back-off may last.
pub const MAX_BACKOFF: Duration = Duration::from_secs(60);
