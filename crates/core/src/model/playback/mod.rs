//! What is playing, and how to change it.

// pub mod local;
use std::time::Duration;

use super::{ContextUri, Device, Track, TrackUri};

/// A snapshot of the Spotify Connect session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playback {
    /// The current item, if any. A device can be active with nothing loaded.
    pub item: Option<Track>,
    /// Whether audio is advancing.
    pub is_playing: bool,
    /// Position reported by the last poll.
    pub progress: Duration,
    /// The device holding playback.
    pub device: Option<Device>,
    /// Repeat mode.
    pub repeat: RepeatMode,
    /// Whether shuffle is on.
    pub shuffle: bool,
    /// Volume of the active device, when it reports one.
    pub volume: Option<u8>,
    /// What is being played from: an album, playlist, or artist.
    pub context: Option<ContextUri>,
}

impl Playback {
    /// Length of the current item.
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.item.as_ref().map(|track| track.duration)
    }
}

/// Repeat mode, mirroring Spotify's three states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepeatMode {
    /// Stop at the end of the context.
    #[default]
    Off,
    /// Loop the current item.
    Track,
    /// Loop the album or playlist.
    Context,
}

impl RepeatMode {
    /// The next mode in the cycle the user sees when pressing the repeat key.
    #[must_use]
    pub const fn cycled(self) -> Self {
        match self {
            Self::Off => Self::Context,
            Self::Context => Self::Track,
            Self::Track => Self::Off,
        }
    }

    /// Short label for the player bar.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Off => "repeat off",
            Self::Track => "repeat one",
            Self::Context => "repeat all",
        }
    }

    /// Whether the indicator should be drawn as active.
    #[must_use]
    pub const fn is_on(self) -> bool {
        !matches!(self, Self::Off)
    }
}

/// What to start playing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayTarget {
    /// Continue whatever is already loaded.
    Resume,
    /// Play an album, playlist, or artist, optionally starting at one item.
    Context {
        /// The context to play.
        uri: ContextUri,
        /// Item within the context to start from.
        start_at: Option<TrackUri>,
    },
    /// Play an explicit list of items, ignoring any context.
    Items(Vec<TrackUri>),
}
