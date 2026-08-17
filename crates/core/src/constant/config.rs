use std::time::Duration;

/// Commented starter file written on first run.
pub const DEFAULT_TEMPLATE: &str = include_str!("../../../../assets/config.default.toml");

/// URL of the dashboard where users register an app.
pub const DASHBOARD_URL: &str = "https://developer.spotify.com/dashboard";

/// File librespot stores its reusable credential in, under [`CACHE_SUBDIR`].
pub const CREDENTIALS_FILE: &str = "credentials.json";

/// How long to wait after a change before writing.
pub const DEBOUNCE: Duration = Duration::from_secs(2);

/// Header written above the values, for anyone who finds the file.
pub const HEADER: &str = "\
# Written by termusic. Edit config.toml instead — this file is overwritten
# whenever you change the volume or the theme, and deleting it is safe.
";
