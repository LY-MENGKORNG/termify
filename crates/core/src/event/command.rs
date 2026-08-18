//! Outbound requests to the Spotify worker.

use std::time::Duration;

use crate::model::{Collection, DeviceId, More, PlayTarget, RepeatMode, Track};

/// A request for the worker to perform against the Spotify API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Re-read playback state and the device list.
    Refresh,
    /// Re-read playback state only. Used for the poll loop.
    Snapshot,
    /// Re-read the device list only.
    Devices,
    /// Poll shortly, to pick up a change made outside the Web API.
    Settle,

    /// Start or resume playback.
    Play(PlayTarget),
    /// Pause playback.
    Pause,
    /// Skip forward.
    Next,
    /// Skip backward.
    Previous,
    /// Seek to an absolute position.
    Seek(Duration),
    /// Set the volume as a percentage.
    Volume(u8),
    /// Set the repeat mode.
    Repeat(RepeatMode),
    /// Set shuffle on or off.
    Shuffle(bool),
    /// Move playback to another device.
    Transfer {
        /// Target device.
        device: DeviceId,
        /// Whether to start playing on arrival.
        play: bool,
    },

    /// Read the saved playlists, albums and artists.
    Library,
    /// Search the catalogue.
    Search(String),
    /// Read what was played recently.
    RecentlyPlayed,
    /// Read what is playing and what follows it.
    Queue,
    /// Read what is inside a collection, for its own page.
    Open(Collection),
    /// Look up lyrics for a track.
    Lyrics(Box<Track>),
    /// Read the next page of a collection already open.
    LoadMore {
        /// The collection being read.
        collection: Collection,
        /// Where the next page starts, as the last one reported.
        from: More,
    },

    /// Stop the worker. Sent when the application is shutting down.
    Shutdown,
}

impl Command {
    /// Whether this command changes remote state.
    #[must_use]
    pub const fn is_mutation(&self) -> bool {
        !matches!(
            self,
            Self::Refresh
                | Self::Snapshot
                | Self::Devices
                | Self::Settle
                | Self::Library
                | Self::Search(_)
                | Self::RecentlyPlayed
                | Self::Queue
                | Self::Open(_)
                | Self::Lyrics(_)
                | Self::LoadMore { .. }
                | Self::Shutdown
        )
    }
}
