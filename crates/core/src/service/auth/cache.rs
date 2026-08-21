//! Keeping the cached token usable.

use std::time::Duration;

use rspotify::prelude::BaseClient;
use rspotify::{AuthCodePkceSpotify, Token};

use crate::error::AuthError;

use super::harden_token_file;

/// How long before expiry a token is renewed.
const REFRESH_MARGIN: Duration = Duration::from_secs(300);

/// Whether [`ensure_fresh`] had to do anything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Renewal {
    /// The token had plenty of life left.
    Current,
    /// A new access token was obtained and cached.
    Renewed,
}

/// Renews the access token when it is close to expiring.
pub async fn ensure_fresh(client: &AuthCodePkceSpotify) -> Result<Renewal, AuthError> {
    let slot = client.get_token();

    let current = {
        let guard = slot.lock().await.map_err(|_| AuthError::NoRefreshToken)?;
        guard
            .as_ref()
            .map(|token| (token.expires_at, token.refresh_token.clone()))
    };

    // `None` here means a previous renewal already failed and cleared it.
    let Some((expires_at, previous)) = current else {
        return Err(AuthError::NoRefreshToken);
    };

    if !due(expires_at) {
        return Ok(Renewal::Current);
    }

    let Some(previous) = previous else {
        return Err(AuthError::NoRefreshToken);
    };

    // `refetch_token` rather than `refresh_token`, because the latter assigns
    // the result unconditionally — including over the refresh token.
    let fetched = client
        .refetch_token()
        .await
        .map_err(|source| AuthError::Refresh {
            source: Box::new(source),
        })?;

    let Some(fetched) = fetched else {
        return Err(AuthError::NoRefreshToken);
    };
    let token = carry_refresh_token(fetched, previous);

    {
        let mut guard = slot.lock().await.map_err(|_| AuthError::NoRefreshToken)?;
        *guard = Some(token);
    }

    client
        .write_token_cache()
        .await
        .map_err(|source| AuthError::Refresh {
            source: Box::new(source),
        })?;
    // The cache was just rewritten, so the mode has to be reapplied — it still
    // holds a refresh token, which is a long-lived credential.
    harden_token_file(&client.get_config().cache_path);

    tracing::info!("renewed the Spotify access token");
    Ok(Renewal::Renewed)
}

/// Carries the previous refresh token forward when a renewal leaves it out.
fn carry_refresh_token(mut renewed: Token, previous: String) -> Token {
    if renewed.refresh_token.is_none() {
        renewed.refresh_token = Some(previous);
    }
    renewed
}

/// Whether a token expiring at `expires_at` should be renewed now.
fn due(expires_at: Option<chrono::DateTime<chrono::Utc>>) -> bool {
    let Some(expires_at) = expires_at else {
        return true;
    };
    let margin = chrono::TimeDelta::from_std(REFRESH_MARGIN).unwrap_or_else(|_| {
        // Unreachable for a five-minute constant, but this crate never panics.
        chrono::TimeDelta::zero()
    });
    chrono::Utc::now() + margin >= expires_at
}
