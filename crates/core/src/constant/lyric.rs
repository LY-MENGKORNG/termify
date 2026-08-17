use std::time::Duration;

/// Endpoint that matches a track and returns its lyrics.
pub const ENDPOINT: &str = "https://lrclib.net/api/get";

/// How long to wait before giving up on a lookup.
pub const TIMEOUT: Duration = Duration::from_secs(6);
