//! Playable items.

use std::time::Duration;

use crate::model::{album::AlbumRef, artist::ArtistRef, identifier::TrackUri};

/// A track or a podcast episode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    /// `spotify:track:…` or `spotify:episode:…`. Absent for local files.
    pub uri: Option<TrackUri>,
    /// Title.
    pub name: String,
    /// Credited artists, or the show for an episode. May be empty.
    pub artists: Vec<ArtistRef>,
    /// Parent album. Always `None` for episodes.
    pub album: Option<AlbumRef>,
    /// Total length.
    pub duration: Duration,
    /// Whether Spotify flags the item as explicit.
    pub explicit: bool,
}

impl Track {
    /// Iterates artist names without allocating a joined string.
    pub fn artist_names(&self) -> impl Iterator<Item = &str> {
        self.artists.iter().map(|artist| artist.name.as_str())
    }

    /// Album name, when there is one.
    #[must_use]
    pub fn album_name(&self) -> Option<&str> {
        self.album.as_ref().map(|album| album.name.as_str())
    }
}
