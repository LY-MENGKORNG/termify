//! Identifier newtypes.

use std::fmt;

/// Opaque Spotify device identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceId(String);

impl DeviceId {
    /// Wraps a raw identifier.
    #[must_use]
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    /// The underlying string, for the API layer.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// URI of something playable as a context: an album, playlist, or artist.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextUri(String);

impl ContextUri {
    /// Wraps a raw `spotify:…` URI.
    #[must_use]
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    /// The underlying string, for the API layer.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The kind of context, parsed out of the URI for display purposes.
    #[must_use]
    pub fn kind(&self) -> Option<&str> {
        self.0.split(':').nth(1)
    }
}

impl fmt::Display for ContextUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// URI of a single playable item: a track or an episode.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrackUri(String);

impl TrackUri {
    /// Wraps a raw `spotify:…` URI.
    #[must_use]
    pub fn new(raw: impl Into<String>) -> Self {
        Self(raw.into())
    }

    /// The underlying string, for the API layer.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TrackUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
