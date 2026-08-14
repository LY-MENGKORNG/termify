//! App configuration.

pub mod constant;
pub mod err;
pub mod paths;
pub mod spotify;

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::config::{constant::DEFAULT_TEMPLATE, err::ConfErr, paths::Paths, spotify::SpotifyConf};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Conf {
    /// Credentials and API behaviour.
    spotify: SpotifyConf,
}

impl Conf {
    /// Loads and validates configuration, creating a starter file when absent.
    ///
    /// A freshly created file is reported as [`ConfigError::CreatedTemplate`]
    /// rather than silently accepted: without a client ID there is nothing the
    /// application can usefully do, and the user needs to be told where to look.
    pub fn load(paths: &Paths) -> Result<Self, ConfErr> {
        let path = paths.config_file();

        if !path.exists() {
            Self::write_template(&path)?;
            return Err(ConfErr::CreatedTemplate { path });
        }

        // Read config path
        let raw = fs::read_to_string(&path).map_err(|source| ConfErr::Io {
            path: path.clone(),
            source,
        })?;

        // Deserialize the raw config
        let conf: Conf = toml::from_str(&raw).map_err(|source| ConfErr::Parse {
            path: path.clone(),
            source: Box::new(source),
        })?;

        conf.validate(&path)?;
        Ok(conf)
    }

    /// Rejects configurations that cannot possibly authenticate.
    ///
    /// Done eagerly at startup: a bad redirect URI otherwise surfaces as an
    /// opaque `INVALID_CLIENT` page in the user's browser. `config_path` is
    /// only used to tell the user which file to edit.
    pub fn validate(&self, path: &Path) -> Result<(), ConfErr> {
        self.spotify.validate(path)
    }

    fn write_template(path: &Path) -> Result<(), ConfErr> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| ConfErr::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        fs::write(path, DEFAULT_TEMPLATE).map_err(|source| ConfErr::Io {
            path: path.to_path_buf(),
            source,
        })
    }
}
