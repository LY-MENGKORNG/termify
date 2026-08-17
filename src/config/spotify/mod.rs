//! Configuration for Spotify

pub mod scope;

use serde::{Deserialize, Serialize};
use std::{path::Path, time::Duration};

use crate::{config::err::ConfErr, utils::url::redirect_port};

/// Spotify credentials and polling behaviour.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct SpotifyConf {
    /// Client ID of your app from the Spotify developer dashboard.
    pub client_id: String,
    /// Redirect URI registered for the app.
    pub redirect_uri: String,
    /// How often to poll `/me/player` while something is playing.
    pub poll_playing_secs: u64,
    /// How often to poll while paused or idle. Longer, to respect rate limits.
    pub poll_idle_secs: u64,
}

impl SpotifyConf {
    /// Interval between polls while audio is playing.
    #[must_use]
    pub fn poll_playing(&self) -> Duration {
        Duration::from_secs(self.poll_playing_secs.max(1))
    }

    /// Interval between polls while paused or idle.
    #[must_use]
    pub fn poll_idle(&self) -> Duration {
        Duration::from_secs(self.poll_idle_secs.max(1))
    }

    pub fn validate(&self, config_path: &Path) -> Result<(), ConfErr> {
        if self.client_id.trim().is_empty() {
            return Err(ConfErr::MissingClientId {
                path: config_path.to_path_buf(),
            });
        }

        let uri = self.redirect_uri.trim();
        if uri.is_empty() {
            return Err(ConfErr::MissingRedirectUri {
                path: config_path.to_path_buf(),
            });
        }

        // Spotify tightened redirect-URI validation in 2025: loopback must be
        // an IP literal. Catching it here saves a confusing round trip.
        let is_loopback_literal = uri.starts_with("http://127.0.0.1")
            || uri.starts_with("http://[::1]")
            || uri.starts_with("https://127.0.0.1")
            || uri.starts_with("https://[::1]");

        if !is_loopback_literal {
            return Err(ConfErr::RedirectUriNotLoopback {
                uri: uri.to_owned(),
            });
        }

        if redirect_port(uri).is_none() {
            return Err(ConfErr::RedirectUriWithoutPort {
                uri: uri.to_owned(),
            });
        }

        Ok(())
    }

    /// Port termusic should listen on for the OAuth callback.
    #[must_use]
    pub fn callback_port(&self) -> Option<u16> {
        redirect_port(self.redirect_uri.trim())
    }
}

impl Default for SpotifyConf {
    fn default() -> Self {
        Self {
            client_id: String::new(),
            redirect_uri: "http://127.0.0.1:8888/callback".to_owned(),
            poll_playing_secs: 5,
            poll_idle_secs: 15,
        }
    }
}
