//! Things that hold tracks.

use super::ContextUri;

/// Which kind of container this is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionKind {
    /// The user's saved tracks.
    LikedSongs,
    /// A playlist, owned or followed.
    Playlist,
    /// An album.
    Album,
    /// An artist, standing in for their top tracks.
    Artist,
}

impl CollectionKind {
    /// Word shown in the type column.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::LikedSongs => "Songs",
            Self::Playlist => "Playlist",
            Self::Album => "Album",
            Self::Artist => "Artist",
        }
    }

    /// Single-column glyph, matching the sidebar's restraint about symbols.
    #[must_use]
    pub const fn glyph(self) -> &'static str {
        match self {
            Self::LikedSongs => "♥",
            Self::Playlist => "≡",
            Self::Album => "◉",
            Self::Artist => "◍",
        }
    }
}

/// Something that can be opened to reveal tracks, and played as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    /// What sort of thing this is.
    pub kind: CollectionKind,
    /// Display name.
    pub name: String,
    /// `spotify:playlist:…`, `spotify:album:…`, `spotify:artist:…`.
    pub uri: Option<ContextUri>,
    /// One line of supporting detail: the owner, the artists, the track count.
    pub subtitle: String,
}

impl Collection {
    /// The user's saved tracks, which no endpoint returns as a collection.
    #[must_use]
    pub fn liked_songs() -> Self {
        Self {
            kind: CollectionKind::LikedSongs,
            name: "Liked Songs".to_owned(),
            uri: None,
            subtitle: "Everything you have saved".to_owned(),
        }
    }

    /// Builds a collection.
    #[must_use]
    pub fn new(
        kind: CollectionKind,
        name: impl Into<String>,
        uri: Option<ContextUri>,
        subtitle: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            name: name.into(),
            uri,
            subtitle: subtitle.into(),
        }
    }

    /// Whether this can be handed to the playback endpoint as a context.
    #[must_use]
    pub const fn is_context(&self) -> bool {
        self.uri.is_some()
    }
}
