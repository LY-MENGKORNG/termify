use std::time::Duration;

/// How long to wait after a change before writing.
pub const DEBOUNCE: Duration = Duration::from_secs(2);

/// Header written above the values, for anyone who finds the file.
pub const HEADER: &str = "\
# Written by termify. Edit config.toml instead, this file is overwritten
# whenever you change the volume or the theme, and deleting it is safe.
";
