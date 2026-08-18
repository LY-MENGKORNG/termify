use crate::{
    error::PlaybackError,
    model::{
        Collection, CollectionPage, Device, Library, Playback, Queue, SearchResults, Track,
        TrackUri,
    },
    service::lyric::lrc::Lyrics,
};

/// A reply from the Spotify worker.
#[derive(Debug, Clone)]
pub enum SpotifyEvent {
    /// A fresh playback snapshot. `None` means no device is active.
    Playback(Option<Box<Playback>>),
    /// A fresh device list.
    Devices(Vec<Device>),

    /// The saved playlists, albums and artists.
    Library(Box<Library>),
    /// Results for a search.
    SearchResults {
        /// The query these answer.
        query: String,
        /// What matched.
        results: Box<SearchResults>,
    },
    /// Recently played tracks, newest first.
    RecentlyPlayed(Vec<Track>),
    /// What is playing and what follows it.
    Queue(Box<Queue>),
    /// The first page of a collection that was opened.
    Opened {
        /// The collection that was opened.
        collection: Box<Collection>,
        /// What is inside it, and whether there is more.
        page: Box<CollectionPage>,
    },
    /// A further page of the collection already open.
    MoreItems {
        /// The collection the page belongs to.
        collection: Box<Collection>,
        /// The rows, and whether there is more after them.
        page: Box<CollectionPage>,
    },

    /// Lyrics for a track, or `None` when the database has none.
    Lyrics {
        /// The track they belong to.
        track: TrackUri,
        /// The words, when there are any.
        lyrics: Option<Box<Lyrics>>,
    },
    /// A lyrics lookup failed.
    LyricsFailed {
        /// The track it was for.
        track: TrackUri,
    },

    /// A request failed in a way the user should know about.
    Failed(PlaybackError),
}
