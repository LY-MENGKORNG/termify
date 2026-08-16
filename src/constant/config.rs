/// Commented starter file written on first run.
pub const DEFAULT_TEMPLATE: &str = include_str!("../../assets/config.default.toml");

/// URL of the dashboard where users register an app.
pub const DASHBOARD_URL: &str = "https://developer.spotify.com/dashboard";

/// Directory name used under every base directory.
pub const APP_DIR: &str = "termify";

/// Overrides the resolved configuration directory.
pub const CONFIG_DIR_ENV: &str = "TERMIFY_CONFIG_DIR";

/// Overrides the resolved cache directory.
pub const CACHE_DIR_ENV: &str = "TERMIFY_CACHE_DIR";

/// Overrides the resolved state directory.
pub const STATE_DIR_ENV: &str = "TERMIFY_STATE_DIR";
