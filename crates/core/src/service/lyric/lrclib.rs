//! Where lyrics come from.

use serde::Deserialize;

use crate::{
    constant::{ENDPOINT, TIMEOUT},
    error::lyric::LyricsError,
    model::track::Track,
    service::lyric::lrc::Lyrics,
};

/// A client for the lyrics database.
#[derive(Debug, Clone)]
pub struct LyricsSource {
    http: reqwest::Client,
}

impl LyricsSource {
    /// Builds a client, or nothing if one cannot be created.
    #[must_use]
    pub fn new() -> Option<Self> {
        // lrclib asks clients to identify themselves, which is only polite for a
        // free service.
        let http = reqwest::Client::builder()
            .user_agent(concat!(
                "termify/",
                env!("CARGO_PKG_VERSION"),
                " (https://crates.io/crates/termify)"
            ))
            .timeout(TIMEOUT)
            .build()
            .ok()?;

        Some(Self { http })
    }

    /// Looks up lyrics for `track`.
    pub async fn fetch(&self, track: &Track) -> Result<Option<Lyrics>, LyricsError> {
        let artist = track.artist_names().collect::<Vec<_>>().join(", ");
        let seconds = track.duration.as_secs().to_string();

        let mut query = vec![
            ("track_name", track.name.as_str()),
            ("artist_name", artist.as_str()),
            // The length lets the database choose between recordings; without it a
            // live version's timings can come back for the studio cut.
            ("duration", seconds.as_str()),
        ];
        if let Some(album) = track.album_name() {
            query.push(("album_name", album));
        }

        let response = self
            .http
            .get(ENDPOINT)
            .query(&query)
            .send()
            .await
            .map_err(|error| {
                tracing::debug!(%error, "lyrics lookup failed");
                LyricsError::Unreachable
            })?;

        let status = response.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            return Err(LyricsError::Unexpected {
                status: status.as_u16(),
            });
        }

        let body = response.text().await.map_err(|error| {
            tracing::debug!(%error, "could not read the lyrics reply");
            LyricsError::Unreachable
        })?;

        let found = serde_json::from_str::<Found>(&body).map_err(|error| {
            tracing::warn!(%error, "could not parse the lyrics reply");
            LyricsError::Malformed
        })?;

        Ok(found.into_lyrics())
    }
}

/// The shape of a successful reply, reduced to what is used.
#[derive(Debug, Deserialize)]
struct Found {
    #[serde(default)]
    instrumental: bool,
    #[serde(default, rename = "syncedLyrics")]
    synced: Option<String>,
    #[serde(default, rename = "plainLyrics")]
    plain: Option<String>,
}

impl Found {
    /// Prefers synced lyrics, falling back to plain ones.
    fn into_lyrics(self) -> Option<Lyrics> {
        if self.instrumental {
            return None;
        }

        if let Some(synced) = self
            .synced
            .as_deref()
            .filter(|text| !text.trim().is_empty())
        {
            let lyrics = Lyrics::parse(synced);
            if !lyrics.is_empty() {
                return Some(lyrics);
            }
        }

        // Readable but not followable — better than an empty pane.
        let plain = self
            .plain
            .as_deref()
            .filter(|text| !text.trim().is_empty())?;
        Some(Lyrics::plain(plain))
    }
}
