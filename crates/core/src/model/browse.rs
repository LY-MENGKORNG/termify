//! What the browsing pages display.

use super::{Collection, More, Track};

/// The user's saved music.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library {
    /// Playlists the user owns or follows.
    pub playlists: Vec<Collection>,
    /// Saved albums.
    pub albums: Vec<Collection>,
    /// Followed artists.
    pub artists: Vec<Collection>,
}

impl Library {
    /// Whether Spotify returned nothing at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.playlists.is_empty() && self.albums.is_empty() && self.artists.is_empty()
    }
}

/// Search results, grouped by kind.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SearchResults {
    /// Matching tracks.
    pub tracks: Vec<Track>,
    /// Matching albums.
    pub albums: Vec<Collection>,
    /// Matching artists.
    pub artists: Vec<Collection>,
    /// Matching playlists.
    pub playlists: Vec<Collection>,
    /// How many songs matched in total, when Spotify says.
    pub track_total: Option<u32>,
}

impl SearchResults {
    /// Whether nothing matched.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
            && self.albums.is_empty()
            && self.artists.is_empty()
            && self.playlists.is_empty()
    }
}

/// What is inside a [`Collection`] once it is opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionItems {
    /// Songs, for a playlist, an album, or the saved tracks.
    Tracks(Vec<Track>),
    /// Releases, for an artist.
    Collections(Vec<Collection>),
}

impl CollectionItems {
    /// How many rows this will produce.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Tracks(tracks) => tracks.len(),
            Self::Collections(collections) => collections.len(),
        }
    }

    /// Whether the collection turned out to be empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// One page of a collection's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionPage {
    /// The rows on this page.
    pub items: CollectionItems,
    /// How many the collection holds, when Spotify says.
    pub total: Option<u32>,
    /// Where the next page starts, or `None` at the end.
    pub more: Option<More>,
}

impl CollectionPage {
    /// A collection that fits in one page.
    #[must_use]
    pub fn only(items: CollectionItems) -> Self {
        let total = u32::try_from(items.len()).ok();

        Self {
            items,
            total,
            more: None,
        }
    }
}

/// What is playing and what follows it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Queue {
    /// The item playing now, if any.
    pub current: Option<Track>,
    /// Items after it, in order.
    pub next: Vec<Track>,
}

impl Queue {
    /// Whether there is nothing to show.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.current.is_none() && self.next.is_empty()
    }
}
