/// Subdirectory of the cache holding librespot's credential and volume cache.
///
/// Named here rather than in [`device`] so that `--logout` can clear it in a
/// build that has local playback compiled out — a stale credential outlives the
/// feature flag that wrote it.
pub const CACHE_SUBDIR: &str = "librespot";

/// File librespot stores its reusable credential in, under [`CACHE_SUBDIR`].
///
/// Named separately so that signing out can drop the credential *without*
/// taking the device identity sitting beside it. That id is not a credential,
/// and discarding it makes the next launch register as a new device, leaving a
/// phantom copy of termify in everyone's device list.
pub const CREDENTIALS_FILE: &str = "credentials.json";
