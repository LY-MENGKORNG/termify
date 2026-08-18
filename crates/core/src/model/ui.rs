use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Appearance and timing.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct UiConfig {
    /// Theme name: a built-in, or the stem of a file in `themes/`.
    pub theme: String,
    /// Animation clock period. Drives progress and marquee, not repaints.
    pub tick_ms: u64,
    /// Width of the expanded sidebar, in columns.
    pub sidebar_width: u16,
    /// How far `<` and `>` seek.
    pub seek_step_secs: u64,
    /// How much `-` and `+` change the volume, in percentage points.
    pub volume_step: u8,
    /// Whether to look up lyrics for the playing track.
    pub lyrics: bool,
    /// Whether to draw a spectrum while termify itself is playing.
    pub visualizer: bool,
    /// How that spectrum is drawn. Also switchable while running.
    pub visualizer_style: VisualizerStyle,
}

impl UiConfig {
    /// Animation clock period.
    #[must_use]
    pub fn tick(&self) -> Duration {
        Duration::from_millis(self.tick_ms.clamp(16, 1000))
    }

    /// Seek distance for a single keypress.
    #[must_use]
    pub fn seek_step(&self) -> Duration {
        Duration::from_secs(self.seek_step_secs.clamp(1, 600))
    }

    /// Volume change for a single keypress.
    #[must_use]
    pub fn volume_step(&self) -> u8 {
        self.volume_step.clamp(1, 50)
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_owned(),
            tick_ms: 250,
            sidebar_width: 22,
            seek_step_secs: 5,
            volume_step: 5,
            lyrics: true,
            visualizer: true,
            visualizer_style: VisualizerStyle::default(),
        }
    }
}

/// How the spectrum is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VisualizerStyle {
    /// Dots mirrored around a centre axis, coloured by the theme's gradient.
    #[default]
    Mirror,
    /// Block bars growing up from the floor, in the accent color.
    Bars,
}

impl VisualizerStyle {
    /// Every style, in the order [`Self::cycled`] walks them.
    pub const ALL: [Self; 2] = [Self::Mirror, Self::Bars];

    /// The next style, wrapping around.
    #[must_use]
    pub const fn cycled(self) -> Self {
        match self {
            Self::Mirror => Self::Bars,
            Self::Bars => Self::Mirror,
        }
    }

    /// Name shown when the style changes, and used in `config.toml`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mirror => "mirror",
            Self::Bars => "bars",
        }
    }
}
