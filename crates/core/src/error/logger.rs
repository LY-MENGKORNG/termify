use std::{io, path::PathBuf};
use thiserror::Error;

/// Something went wrong while preparing the log file.
#[derive(Debug, Error)]
pub enum LogError {
    /// open the log
    #[error("could not open log file {path}: {source}")]
    Open {
        /// Buffer's path to the log
        path: PathBuf,
        /// What to write in the log
        source: io::Error,
    },

    /// Message for existiting tracing subscriber
    #[error("a tracing subscriber is already installed")]
    AlreadyInstalled,
}
