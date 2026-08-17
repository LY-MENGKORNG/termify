use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    sync::Arc,
};

use tracing_subscriber::EnvFilter;

use crate::{
    constant::logger::{DEFAULT_FILTER, FILTER_ENV_LOG},
    error::logger::LogError,
};

pub fn init(log_file: &Path) -> Result<PathBuf, LogError> {
    let file = open_append(log_file)?;

    let filter = EnvFilter::try_from_env(FILTER_ENV_LOG)
        .or_else(|_| EnvFilter::try_new(DEFAULT_FILTER))
        .unwrap_or_default();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .with_target(true)
        .with_writer(Arc::new(file))
        .try_init()
        .map_err(|_| LogError::AlreadyInstalled)?;

    Ok(log_file.to_path_buf())
}

fn open_append(path: &Path) -> Result<File, LogError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| LogError::Open {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|source| LogError::Open {
            path: path.to_path_buf(),
            source,
        })
}
