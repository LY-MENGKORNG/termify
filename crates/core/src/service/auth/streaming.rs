//! The credential librespot streams with.

use std::collections::HashSet;
use std::path::Path;

use rspotify::prelude::OAuthClient;
use rspotify::{AuthCodePkceSpotify, Credentials, OAuth, scopes};

use crate::constant::CLIENT_ID;

use super::AuthError;

/// Redirect URI registered against [`CLIENT_ID`].
fn redirect_uri(port: u16) -> String {
    format!("http://127.0.0.1:{port}/login")
}

/// What the streaming credential is allowed to do.
fn required_scopes() -> HashSet<String> {
    scopes!("streaming")
}

/// Obtains a token librespot can open a session with.
pub async fn token<F>(port: u16, cache_path: &Path, announce: F) -> Result<String, AuthError>
where
    F: FnOnce(&str),
{
    let mut client = build_client(port, cache_path);

    let cached = client.read_token_cache(true).await.unwrap_or_else(|error| {
        tracing::warn!(%error, "ignoring an unreadable streaming credential");
        None
    });

    if let Some(cached) = cached {
        let client = AuthCodePkceSpotify::from_token_with_config(
            cached,
            credentials(),
            oauth(port),
            super::rspotify_config(cache_path),
        );

        match super::ensure_fresh(&client).await {
            Ok(_) => {
                super::harden_token_file(cache_path);
                if let Some(token) = super::access_token(&client).await {
                    tracing::info!("reusing the cached streaming credential");
                    return Ok(token);
                }
                tracing::warn!("the cached streaming credential held no access token");
            }
            // Not fatal, and not worth failing local audio over: fall through
            // to a fresh sign-in rather than leaving the user with no way back.
            Err(error) => {
                tracing::warn!(%error, "could not renew the streaming credential");
            }
        }
    }

    super::consent(&mut client, port, announce).await?;
    super::harden_token_file(cache_path);

    let token = super::access_token(&client)
        .await
        .ok_or(AuthError::EmptyToken)?;

    tracing::info!("obtained a streaming credential");
    Ok(token)
}

/// Deletes the cached streaming credential.
pub fn forget(cache_path: &Path) -> std::io::Result<bool> {
    super::forget(cache_path)
}

fn build_client(port: u16, cache_path: &Path) -> AuthCodePkceSpotify {
    AuthCodePkceSpotify::with_config(
        credentials(),
        oauth(port),
        super::rspotify_config(cache_path),
    )
}

fn credentials() -> Credentials {
    Credentials::new_pkce(CLIENT_ID)
}

fn oauth(port: u16) -> OAuth {
    OAuth {
        redirect_uri: redirect_uri(port),
        scopes: required_scopes(),
        ..OAuth::default()
    }
}
