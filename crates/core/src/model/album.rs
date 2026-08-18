//! Albums.

/// The amount of album information that arrives embedded in a track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlbumRef {
    /// Display name.
    pub name: String,
    /// `spotify:album:…`, absent for local files.
    pub uri: Option<String>,
    /// URL of the largest available cover image.
    pub cover_url: Option<String>,
    /// Release year, when Spotify supplies a parseable date.
    pub year: Option<u16>,
}

impl AlbumRef {
    /// Builds a reference with only a name, for fixtures and local files.
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uri: None,
            cover_url: None,
            year: None,
        }
    }
}
