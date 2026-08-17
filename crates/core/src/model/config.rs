use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    config::PlaybackConfig,
    error::config::ConfigError,
    model::{spotify::SpotifyConfig, ui::UiConfig},
};

/// Everything the user can configure.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// Credentials and API behaviour.
    pub spotify: SpotifyConfig,
    /// Where audio comes out.
    pub playback: PlaybackConfig,
    /// Appearance and timing.
    pub ui: UiConfig,
    /// Key rebindings, as `"key" = "action"`.
    pub keys: BTreeMap<String, String>,
}

impl Config {
    /// Rejects configurations that cannot possibly authenticate.
    pub fn validate(&self, config_path: &Path) -> Result<(), ConfigError> {
        self.spotify.validate(config_path)
    }
}
