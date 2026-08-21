use std::time::Duration;

use crate::{
    config::PlaybackConfig,
    model::{UiConfig, VisualizerStyle},
};

/// The numeric knobs the reducer needs, lifted out of [`UiConfig`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    /// Distance a single seek keypress covers.
    pub seek_step: Duration,
    /// Percentage points a single volume keypress covers.
    pub volume_step: u8,
    /// Preferred width of the expanded sidebar.
    pub sidebar_width: u16,
    /// Whether play-with-nothing-loaded falls back to Liked Songs.
    pub autoplay_liked_songs: bool,
    /// Whether to look lyrics up at all.
    pub lyrics: bool,
    /// Whether to fetch cover art at all.
    pub artwork: bool,
    /// How the spectrum is drawn.
    pub visualizer_style: VisualizerStyle,
}

impl Settings {
    /// Derives settings from the user's configuration.
    #[must_use]
    pub fn from_config(ui: &UiConfig, playback: &PlaybackConfig) -> Self {
        Self {
            seek_step: ui.seek_step(),
            volume_step: ui.volume_step(),
            sidebar_width: ui.sidebar_width,
            autoplay_liked_songs: playback.autoplay_liked_songs,
            lyrics: ui.lyrics,
            artwork: ui.artwork,
            visualizer_style: ui.visualizer_style,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::from_config(&UiConfig::default(), &PlaybackConfig::default())
    }
}
