//! OAuth scopes.

use std::collections::HashSet;

use rspotify::scopes;

/// Every scope termusic will ever ask for.
#[must_use]
pub fn required() -> HashSet<String> {
    scopes!(
        // Playback: reading what is playing, and controlling it.
        "user-read-playback-state",
        "user-modify-playback-state",
        "user-read-currently-playing",
        // No `streaming` here: it reads like it belongs, but the token that opens
        // the audio session is a different one. See `spotify::auth::streaming`.
        "user-read-private",
        // Reserved for the library and playlist pages.
        "playlist-read-private",
        "playlist-read-collaborative",
        "user-library-read",
        "user-top-read",
        "user-read-recently-played",
        "user-follow-read"
    )
}
