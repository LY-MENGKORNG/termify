pub mod cache;
pub mod callback;
pub mod pkce;
pub mod scope;
pub mod streaming;

use std::io;
use std::path::{Path, PathBuf};

use rspotify::prelude::{BaseClient, OAuthClient};
use rspotify::{AuthCodePkceSpotify, Config as RspotifyConfig, Credentials, OAuth};

pub use cache::*;
pub use callback::*;
pub use pkce::*;
pub use scope::*;
pub use streaming::*;

use crate::error::{AuthError, Endpoint};
use crate::model::SpotifyConfig;
use crate::service::pkce::consent;

/// An authenticated session.
pub struct Session {
    /// The authenticated client, ready to be wrapped in a controller.
    pub client: AuthCodePkceSpotify,
    /// Display name of the signed-in user, when it could be read.
    pub user: Option<String>,
}

/// Signs in, reusing a cached token when one is still usable.
pub async fn authenticate<F>(
    config: &SpotifyConfig,
    token_path: &Path,
    announce: F,
) -> Result<Session, AuthError>
where
    F: FnOnce(&str),
{
    let port = config.callback_port().ok_or(AuthError::NoCallbackPort)?;
    let mut client = build_client(config, token_path);

    // `read_token_cache` also checks the cached token covers every scope we now
    // require, so widening `scopes::required` re-triggers consent on its own.
    let cached = client.read_token_cache(true).await.unwrap_or_else(|error| {
        tracing::warn!(%error, "ignoring unreadable token cache");
        None
    });

    if let Some(token) = cached {
        let client = AuthCodePkceSpotify::from_token_with_config(
            token,
            credentials(config),
            oauth(config),
            rspotify_config(token_path),
        );

        match profile_name(&client).await {
            Ok(user) => {
                harden_token_file(token_path);
                tracing::info!(%user, "reusing cached Spotify session");
                return Ok(Session {
                    client,
                    user: Some(user),
                });
            }
            // Being unable to reach Spotify says nothing about the token, and a
            // fresh sign-in needs the network that just failed. Keep the session.
            Err(error) if Endpoint::classify(Endpoint::Profile, &error).is_transient() => {
                tracing::warn!(%error, "could not reach Spotify; keeping the cached session");
                harden_token_file(token_path);
                return Ok(Session { client, user: None });
            }
            // Genuinely rejected. Fall through to a full sign-in rather than
            // dying on startup.
            Err(error) => {
                tracing::warn!(%error, "cached token rejected; starting a fresh sign-in");
            }
        }
    }

    consent(&mut client, port, announce).await?;
    harden_token_file(token_path);

    let user = profile_name(&client)
        .await
        .map_err(|source| AuthError::Profile {
            source: Box::new(source),
        })?;

    tracing::info!(%user, "signed in to Spotify");
    Ok(Session {
        client,
        user: Some(user),
    })
}

/// Deletes the cached token, so the next run signs in from scratch.
pub fn forget(token_path: &Path) -> io::Result<bool> {
    match std::fs::remove_file(token_path) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn build_client(config: &SpotifyConfig, token_path: &Path) -> AuthCodePkceSpotify {
    AuthCodePkceSpotify::with_config(
        credentials(config),
        oauth(config),
        rspotify_config(token_path),
    )
}

fn credentials(config: &SpotifyConfig) -> Credentials {
    Credentials::new_pkce(config.client_id.trim())
}

fn oauth(config: &SpotifyConfig) -> OAuth {
    OAuth {
        redirect_uri: config.redirect_uri.trim().to_owned(),
        scopes: required(),
        ..OAuth::default()
    }
}

pub(super) fn rspotify_config(token_path: &Path) -> RspotifyConfig {
    RspotifyConfig {
        cache_path: token_path.to_path_buf(),
        token_cached: true,
        // Deliberately off: rspotify assigns whatever the token endpoint returns,
        // so a response omitting `refresh_token` would erase the one we hold.
        token_refreshing: false,
        ..RspotifyConfig::default()
    }
}

async fn profile_name(client: &AuthCodePkceSpotify) -> Result<String, rspotify::ClientError> {
    let profile = client.me().await?;
    Ok(profile
        .display_name
        .filter(|name| !name.trim().is_empty())
        .unwrap_or_else(|| profile.id.to_string()))
}

/// Restricts the token cache to the current user.
pub(super) fn harden_token_file(token_path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        if let Err(error) =
            std::fs::set_permissions(token_path, std::fs::Permissions::from_mode(0o600))
        {
            tracing::warn!(
                path = %token_path.display(),
                %error,
                "could not restrict permissions on the token cache"
            );
        }
    }

    #[cfg(not(unix))]
    let _ = token_path;
}

/// Path of the token cache, for callers that only have the cache directory.
#[must_use]
pub fn token_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join("token.json")
}
