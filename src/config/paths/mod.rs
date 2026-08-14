pub mod constant;

use std::{
    env,
    path::{Path, PathBuf},
};

use directories::{BaseDirs, ProjectDirs};

use crate::{
    config::{
        err::ConfErr,
        paths::constant::{APP_DIR, CACHE_DIR_ENV, CONFIG_DIR_ENV, STATE_DIR_ENV},
    },
    services::local::constant::{CACHE_SUBDIR, CREDENTIALS_FILE},
};

/// Which platform-default directory to fall back to.
#[derive(Clone, Copy)]
enum DirKind {
    Config,
    Cache,
    State,
}

/// Resolved locations of every file the application reads or writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paths {
    config: PathBuf,
    cache: PathBuf,
    state: PathBuf,
}

impl Paths {
    /// Resolves paths from the environment.
    ///
    /// Precedence: explicit override, then XDG, then `~/.config` (Unix), then
    /// the platform default.
    pub fn resolve() -> Result<Self, ConfErr> {
        let config_dir = Self::resolve_dir(
            CONFIG_DIR_ENV,
            "XDG_CONFIG_HOME",
            ".config",
            DirKind::Config,
        )
        .ok_or(ConfErr::NoHomeDirectory)?;
        let cache_dir =
            Self::resolve_dir(CACHE_DIR_ENV, "XDG_CACHE_HOME", ".cache", DirKind::Cache)
                .ok_or(ConfErr::NoHomeDirectory)?;
        let state_dir = Self::resolve_dir(
            STATE_DIR_ENV,
            "XDG_STATE_HOME",
            ".local/state",
            DirKind::State,
        )
        .ok_or(ConfErr::NoHomeDirectory)?;

        Ok(Self {
            config: config_dir,
            cache: cache_dir,
            state: state_dir,
        })
    }

    /// Builds a set of paths rooted at explicit directories, for tests.
    ///
    /// State lands in the cache root unless [`Self::with_state_dir`] says
    /// otherwise, which keeps the common case to two arguments.
    pub fn with_roots(config_dir: impl Into<PathBuf>, cache_dir: impl Into<PathBuf>) -> Self {
        let cache_dir = cache_dir.into();
        Self {
            config: config_dir.into(),
            state: cache_dir.clone(),
            cache: cache_dir,
        }
    }

    /// Points state at its own directory, for tests that care about the split.
    #[must_use]
    pub fn with_state_dir(mut self, state_dir: impl Into<PathBuf>) -> Self {
        self.state = state_dir.into();
        self
    }

    /// Directory holding `config.toml` and `themes/`.
    #[must_use]
    pub fn config_dir(&self) -> &Path {
        &self.config
    }

    /// Directory holding the token cache and the log file.
    #[must_use]
    pub fn cache_dir(&self) -> &Path {
        &self.cache
    }

    /// Directory holding `state.toml`.
    ///
    /// Separate from the cache because the contents are not reconstructible:
    /// clearing a cache should cost nothing, and forgetting which theme the
    /// user picked is not nothing.
    #[must_use]
    pub fn state_dir(&self) -> &Path {
        &self.state
    }

    /// The main configuration file.
    #[must_use]
    pub fn config_file(&self) -> PathBuf {
        self.config.join("config.toml")
    }

    /// Directory scanned for user-supplied `*.toml` themes.
    #[must_use]
    pub fn themes_dir(&self) -> PathBuf {
        self.config.join("themes")
    }

    /// Cached OAuth token. Contains a refresh token, so it is kept at `0600`.
    #[must_use]
    pub fn token_file(&self) -> PathBuf {
        self.cache.join("token.json")
    }

    /// Cached credential for the streaming session, kept at `0600`.
    ///
    /// Deliberately not the same file as [`token_file`](Self::token_file).
    /// Local playback authorises under a different client id entirely, so the
    /// two credentials expire, refresh, and are revoked independently — sharing
    /// one file would mean renewing either could invalidate the other.
    #[must_use]
    pub fn streaming_token_file(&self) -> PathBuf {
        self.cache.join("streaming.json")
    }

    /// Directory librespot keeps its own credential and audio cache in.
    #[must_use]
    pub fn librespot_dir(&self) -> PathBuf {
        self.cache.join(CACHE_SUBDIR)
    }

    /// The reusable credential librespot writes after a successful login.
    ///
    /// A credential in its own right, and the one that survives clearing
    /// [`token_file`](Self::token_file) — which is what used to make signing
    /// out look like it had done nothing.
    #[must_use]
    pub fn librespot_credentials_file(&self) -> PathBuf {
        self.librespot_dir().join(CREDENTIALS_FILE)
    }

    /// Rolling log file.
    #[must_use]
    pub fn log_file(&self) -> PathBuf {
        self.cache.join("termify.log")
    }

    /// What termify remembers between runs: volume, theme, spectrum style.
    ///
    /// Written by the application, never by hand — unlike
    /// [`config_file`](Self::config_file), which is the other way around.
    #[must_use]
    pub fn state_file(&self) -> PathBuf {
        self.state.join("state.toml")
    }

    fn resolve_dir(
        override_env: &str,
        xdg_env: &str,
        home_subdir: &str,
        kind: DirKind,
    ) -> Option<PathBuf> {
        if let Some(dir) = Self::non_empty(override_env) {
            return Some(dir);
        }

        if let Some(dir) = Self::non_empty(xdg_env) {
            return Some(dir.join(APP_DIR));
        }

        if cfg!(unix) {
            if let Some(base) = BaseDirs::new() {
                return Some(base.home_dir().join(home_subdir).join(APP_DIR));
            }
        }

        let dirs = ProjectDirs::from("", "", APP_DIR)?;
        let dir = match kind {
            DirKind::Config => dirs.config_dir(),
            DirKind::Cache => dirs.cache_dir(),
            // Windows has no state directory of its own; roaming this would sync
            // one machine's volume onto another, so the local data dir it is.
            DirKind::State => dirs.data_local_dir(),
        };
        Some(dir.to_path_buf())
    }

    /// Reads an environment variable, treating an empty value as unset.
    fn non_empty(key: &str) -> Option<PathBuf> {
        let value = env::var_os(key)?;
        if value.is_empty() {
            return None;
        }
        Some(PathBuf::from(value))
    }
}
