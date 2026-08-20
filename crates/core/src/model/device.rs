//! Spotify Connect devices.

use super::DeviceId;

/// A device that can be told to play.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    /// Identifier, absent for devices Spotify declines to name (rare).
    pub id: Option<DeviceId>,
    /// User-visible name, e.g. "Kitchen speaker".
    pub name: String,
    /// Device category, used for a short type label.
    pub kind: DeviceKind,
    /// Whether this device currently holds playback.
    pub is_active: bool,
    /// Whether it is running someone else's session and cannot be taken over.
    pub is_restricted: bool,
    /// Current volume, when the device reports one.
    pub volume: Option<u8>,
}

impl Device {
    /// Whether termify can transfer playback here.
    #[must_use]
    pub fn is_selectable(&self) -> bool {
        self.id.is_some() && !self.is_restricted
    }

    /// Whether volume can be set remotely.
    #[must_use]
    pub fn supports_volume(&self) -> bool {
        self.volume.is_some()
    }
}

/// Device category as reported by Spotify.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    /// Desktop or laptop.
    Computer,
    /// Phone or tablet.
    Smartphone,
    /// Speaker, including cast targets.
    Speaker,
    /// Television or set-top box.
    Tv,
    /// Car head unit.
    Automobile,
    /// Anything else, including types added after this was written.
    Other,
}

impl DeviceKind {
    /// A short lowercase label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Computer => "computer",
            Self::Smartphone => "phone",
            Self::Speaker => "speaker",
            Self::Tv => "tv",
            Self::Automobile => "car",
            Self::Other => "device",
        }
    }
}
