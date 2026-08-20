use serde::{Deserialize, Serialize};

use crate::constant::{DEFAULT_CALLBACK_PORT, DEFAULT_DEVICE_NAME};

/// Where audio actually comes out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackMode {
    /// termify decodes audio itself, appearing as its own Connect device.
    #[default]
    Local,
    /// termify only commands a device running elsewhere.
    Remote,
}

/// Audio output settings.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct PlaybackConfig {
    /// Whether to play audio here or drive another device.
    pub mode: PlaybackMode,
    /// Name termify announces itself under in Spotify's device lists.
    pub device_name: String,
    /// Volume the local device starts at, as a percentage.
    pub initial_volume: u8,
    /// Whether to start Liked Songs when told to play with nothing loaded.
    pub autoplay_liked_songs: bool,
    /// Port to listen on while authorising local playback.
    pub callback_port: u16,
}

impl PlaybackConfig {
    /// Whether local audio was asked for.
    #[must_use]
    pub fn wants_local(&self) -> bool {
        self.mode == PlaybackMode::Local
    }

    /// Initial volume, clamped to a percentage.
    #[must_use]
    pub fn initial_volume(&self) -> u8 {
        self.initial_volume.min(100)
    }

    /// Port to listen on for the local-playback sign-in.
    #[must_use]
    pub fn callback_port(&self) -> u16 {
        if self.callback_port == 0 {
            DEFAULT_CALLBACK_PORT
        } else {
            self.callback_port
        }
    }

    /// Device name, falling back to the default when blank.
    #[must_use]
    pub fn device_name(&self) -> &str {
        let name = self.device_name.trim();
        if name.is_empty() {
            DEFAULT_DEVICE_NAME
        } else {
            name
        }
    }
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self {
            mode: PlaybackMode::default(),
            device_name: DEFAULT_DEVICE_NAME.to_owned(),
            initial_volume: 70,
            autoplay_liked_songs: false,
            callback_port: DEFAULT_CALLBACK_PORT,
        }
    }
}
