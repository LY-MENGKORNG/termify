use serde::{Deserialize, Serialize};

use super::{color::Hex, constant::SPECTRUM_STOPS, dark};

/// The semantic tokens a theme must provide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Palette {
    /// Page background.
    pub bg: Hex,
    /// Raised surfaces: sidebar, player bar.
    pub surface: Hex,
    /// Floating surfaces: modals, palette.
    pub overlay: Hex,
    /// Border and divider lines.
    pub border: Hex,
    /// Background of a selected row.
    pub selection: Hex,

    /// Primary text.
    pub text: Hex,
    /// Secondary text.
    pub subtext: Hex,
    /// Tertiary text and hints.
    pub muted: Hex,

    /// Primary accent.
    pub accent: Hex,
    /// Secondary accent.
    pub accent_alt: Hex,

    /// Success foreground.
    pub success: Hex,
    /// Warning foreground.
    pub warning: Hex,
    /// Error foreground.
    pub error: Hex,

    /// Elapsed progress.
    pub progress_filled: Hex,
    /// Remaining progress.
    pub progress_empty: Hex,

    /// The spectrum's vertical gradient, bottom stop first.
    pub spectrum: [Hex; SPECTRUM_STOPS],
}

impl Default for Palette {
    fn default() -> Self {
        dark::PALETTE
    }
}
