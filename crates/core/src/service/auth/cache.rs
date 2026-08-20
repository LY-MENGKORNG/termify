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

#[cfg(test)]
mod tests {
    use super::*;

    /// A token expiring at `offset` from now.
    fn token_expiring_in(seconds: i64, refresh: Option<&str>) -> Token {
        Token {
            access_token: "access".to_owned(),
            expires_at: chrono::TimeDelta::try_seconds(seconds)
                .map(|delta| chrono::Utc::now() + delta),
            refresh_token: refresh.map(str::to_owned),
            ..Token::default()
        }
    }

    #[test]
    fn a_renewal_without_a_refresh_token_keeps_the_previous_one() {
        // The whole bug: taking this response at face value erases the only
        // credential that can renew the session, permanently.
        let renewed = token_expiring_in(3600, None);

        let carried = carry_refresh_token(renewed, "the-refresh-token".to_owned());

        assert_eq!(carried.refresh_token.as_deref(), Some("the-refresh-token"));
    }

    #[test]
    fn a_renewal_that_brings_its_own_refresh_token_wins() {
        let renewed = token_expiring_in(3600, Some("rotated"));

        let carried = carry_refresh_token(renewed, "stale".to_owned());

        // Spotify rotates these; keeping the old one would fail next time.
        assert_eq!(carried.refresh_token.as_deref(), Some("rotated"));
    }

    #[test]
    fn a_fresh_token_is_left_alone() {
        assert!(!due(token_expiring_in(3600, None).expires_at));
    }

    #[test]
    fn a_token_inside_the_margin_is_renewed_before_it_dies() {
        // The point of the margin: renew while the current token still works,
        // so no request is ever the thing that discovers the expiry.
        assert!(due(token_expiring_in(60, None).expires_at));
        assert!(due(token_expiring_in(-1, None).expires_at));
    }

    #[test]
    fn an_undated_token_is_renewed_rather_than_assumed_good() {
        assert!(due(None));
    }
}
