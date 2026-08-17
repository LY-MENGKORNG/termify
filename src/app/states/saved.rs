//! What termify remembers about itself between runs.

use crate::{constant::state::HEADER, models::ui::VisualizerStyle};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

/// The settings termify carries from one run to the next.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct SavedState {
    /// Volume last set from inside termify, as a percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<u8>,
    /// Theme last chosen in the picker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// Spectrum style last cycled to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualizer_style: Option<VisualizerStyle>,
}

impl SavedState {
    /// Reads remembered state, falling back to remembering nothing.
    #[must_use]
    pub fn load(path: &Path) -> Self {
        let raw = match fs::read_to_string(path) {
            Ok(raw) => raw,
            Err(err) => {
                if err.kind() != io::ErrorKind::NotFound {
                    tracing::warn!(%err, path = %path.display(), "could not read saved state");
                }
                return Self::default();
            }
        };

        match toml::from_str::<Self>(&raw) {
            Ok(saved) => saved.sanitized(),
            Err(err) => {
                tracing::warn!(%err, path = %path.display(), "ignoring unreadable saved state");
                Self::default()
            }
        }
    }

    /// Writes the state, replacing whatever was there.
    pub fn write(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let body = toml::to_string(self).map_err(io::Error::other)?;

        let temporary = path.with_extension("toml.new");
        fs::write(&temporary, format!("{HEADER}{body}"))?;
        // Rename is atomic within a directory, and replaces silently on both
        // Unix and Windows.
        match fs::rename(&temporary, path) {
            Ok(()) => Ok(()),
            Err(error) => {
                let _ = fs::remove_file(&temporary);
                Err(error)
            }
        }
    }

    /// The theme to start in, given what the user configured.
    #[must_use]
    pub fn theme_or<'a>(&'a self, configured: &'a str) -> &'a str {
        self.theme.as_deref().unwrap_or(configured)
    }

    /// The volume to start the local device at, given what the user configured.
    #[must_use]
    pub fn volume_or(&self, configured: u8) -> u8 {
        self.volume.unwrap_or(configured).min(100)
    }

    /// The spectrum style to start with, given what the user configured.
    #[must_use]
    pub fn visualizer_style_or(&self, configured: VisualizerStyle) -> VisualizerStyle {
        self.visualizer_style.unwrap_or(configured)
    }

    /// Clamps anything a hand-edited or older file could have got wrong.
    fn sanitized(mut self) -> Self {
        self.volume = self.volume.map(|volume| volume.min(100));
        // An empty name would resolve to no theme at all and hide the
        // configured one behind a fallback.
        self.theme = self.theme.filter(|name| !name.trim().is_empty());
        self
    }
}
