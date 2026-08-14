use std::{io, path::PathBuf};

use thiserror::Error;

use super::constant::DASHBOARD_URL;
use crate::config::paths::constant::CONFIG_DIR_ENV;

/// Configuration could not be loaded, or is unusable as written.
///
/// Every message is written for the person running the binary, not for a
/// stack trace: it says what is wrong and what to do about it.
#[derive(Debug, Error)]
pub enum ConfErr {
    /// Neither `$HOME` nor a platform directory could be determined.
    #[error(
        "could not determine your home directory; set {} to continue",
        CONFIG_DIR_ENV
    )]
    NoHomeDirectory,
    /// A starter configuration was written; the user must fill it in.
    #[error(
        "wrote a starter configuration to {path}\n\n\
         Next steps:\n  \
         1. Create an app at {DASHBOARD_URL}\n  \
         2. Add  http://127.0.0.1:8888/callback  as a Redirect URI\n  \
         3. Paste the app's Client ID into `client_id` in the file above\n  \
         4. Run termify again"
    )]
    CreatedTemplate {
        /// File that was just created.
        path: PathBuf,
    },

    /// `client_id` is empty.
    #[error(
        "no Spotify client ID configured in {path}\n\n\
         Create an app at {DASHBOARD_URL}, then paste its Client ID into \
         `client_id`.\nPKCE needs no client secret, so there is nothing else to copy."
    )]
    MissingClientId {
        /// Configuration file that needs editing.
        path: PathBuf,
    },

    /// `redirect_uri` is empty.
    #[error("no `redirect_uri` configured in {path}; try http://127.0.0.1:8888/callback")]
    MissingRedirectUri {
        /// Configuration file that needs editing.
        path: PathBuf,
    },

    /// `redirect_uri` is not a loopback IP literal.
    #[error(
        "`redirect_uri` must be a loopback address: {uri}\n\n\
         Spotify rejects `localhost` and non-HTTPS hosts. Use the IP literal, \
         e.g. http://127.0.0.1:8888/callback,\nand register exactly the same \
         value in the dashboard."
    )]
    RedirectUriNotLoopback {
        /// The offending value.
        uri: String,
    },

    /// `redirect_uri` has no port, so there is nothing to listen on.
    #[error("`redirect_uri` needs an explicit port: {uri} (e.g. http://127.0.0.1:8888/callback)")]
    RedirectUriWithoutPort {
        /// The offending value.
        uri: String,
    },

    /// The file could not be read or written.
    #[error("could not access {path}: {source}")]
    Io {
        /// Path involved.
        path: PathBuf,
        /// Underlying I/O failure.
        source: io::Error,
    },

    /// The file is not valid TOML, or has unknown keys.
    #[error("{path} is not valid configuration:\n{source}")]
    Parse {
        /// File that failed to parse.
        path: PathBuf,
        /// Parser diagnostic, which already points at the offending line.
        source: Box<toml::de::Error>,
    },
}
