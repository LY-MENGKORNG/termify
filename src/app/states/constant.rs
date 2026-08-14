use std::time::Duration;

/// How long to wait after a change before writing.
///
/// Holding `+` sends a keypress per repeat; without this, each one would be a
/// separate write. Every field here is cheap to lose, so trading a couple of
/// seconds of durability for one write per burst is the right way round.
const DEBOUNCE: Duration = Duration::from_secs(2);

/// Header written above the values, for anyone who finds the file.
pub const HEADER: &str = "\
# Written by termify. Edit config.toml instead — this file is overwritten
# whenever you change the volume or the theme, and deleting it is safe.
";
