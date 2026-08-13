//! The logger service

pub mod constant;
pub mod err;

use std::{
    fs::{self, File, OpenOptions},
    path::{Path, PathBuf},
    sync::Arc,
};

use tracing_subscriber::EnvFilter;

use crate::services::logger::{
    constant::{DEFAULT_FILTER, FILTER_ENV_LOG},
    err::LogErr,
};

pub struct Logger {}

impl Logger {
    pub fn init(log_file: &Path) -> Result<PathBuf, LogErr> {
        let file = Self::open_append(log_file)?;

        let filter = EnvFilter::try_from_env(FILTER_ENV_LOG)
            .or_else(|_| EnvFilter::try_new(DEFAULT_FILTER))
            .unwrap_or_default();

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_ansi(false)
            .with_target(true)
            .with_writer(Arc::new(file))
            .try_init()
            .map_err(|_| LogErr::AlreadyInstalled)?;

        Ok(log_file.to_path_buf())
    }

    fn open_append(path: &Path) -> Result<File, LogErr> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| LogErr::Open {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|source| LogErr::Open {
                path: path.to_path_buf(),
                source,
            })
    }
}
