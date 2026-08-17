//! Artists.

/// The amount of artist information that arrives embedded in a track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtistRef {
    /// Display name.
    pub name: String,
    /// `spotify:artist:…`, absent for local files.
    pub uri: Option<String>,
}

impl ArtistRef {
    /// Builds a reference with no URI, for fixtures and local files.
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            uri: None,
        }
    }
}
