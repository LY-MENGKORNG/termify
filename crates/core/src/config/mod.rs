pub mod error;
pub mod paths;

use std::{fs, path::Path};

pub use error::*;
pub use paths::*;

use crate::{constant::DEFAULT_TEMPLATE, model::config::Config};

/// Loads and validates configuration, creating a starter file when absent.
pub fn load(paths: &Paths) -> Result<Config, ConfigError> {
    let path = paths.config_file();

    if !path.exists() {
        write_template(&path)?;
        return Err(ConfigError::CreatedTemplate { path });
    }

    let raw = fs::read_to_string(&path).map_err(|source| ConfigError::Io {
        path: path.clone(),
        source,
    })?;

    let config: Config = toml::from_str(&raw).map_err(|source| ConfigError::Parse {
        path: path.clone(),
        source: Box::new(source),
    })?;

    config.validate(&path)?;
    Ok(config)
}

fn write_template(path: &Path) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| ConfigError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(path, DEFAULT_TEMPLATE).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })
}
