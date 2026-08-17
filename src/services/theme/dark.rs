//! The default built-in theme.

use super::color::Hex;
use super::palette::Palette;

/// Name reported by the built-in theme.
pub const NAME: &str = "dark";

/// Default palette: near-black surfaces, one saturated accent, muted rest.
///
/// The scale is deliberately shallow — four greys carry every surface — so that
/// depth comes from spacing rather than from boxes and dividers.
pub const PALETTE: Palette = Palette {
    bg: Hex::rgb(0x0d, 0x0f, 0x12),
    surface: Hex::rgb(0x14, 0x17, 0x1a),
    overlay: Hex::rgb(0x1b, 0x1f, 0x24),
    border: Hex::rgb(0x26, 0x2b, 0x31),
    selection: Hex::rgb(0x1f, 0x27, 0x33),

    text: Hex::rgb(0xe6, 0xe9, 0xec),
    subtext: Hex::rgb(0xa8, 0xb0, 0xb8),
    muted: Hex::rgb(0x6b, 0x74, 0x7d),

    accent: Hex::rgb(0x1e, 0xd7, 0x60),
    accent_alt: Hex::rgb(0x7a, 0xa2, 0xf7),

    success: Hex::rgb(0x1e, 0xd7, 0x60),
    warning: Hex::rgb(0xe0, 0xaf, 0x68),
    error: Hex::rgb(0xf7, 0x76, 0x8e),

    progress_filled: Hex::rgb(0x1e, 0xd7, 0x60),
    progress_empty: Hex::rgb(0x2a, 0x30, 0x38),

    // Warm at the bottom, cool at the top: the mirrored spectrum reads as one
    // gradient through the centre axis rather than as two halves.
    spectrum: [
        Hex::rgb(0xd3, 0x3f, 0x9d),
        Hex::rgb(0xf7, 0x76, 0x8e),
        Hex::rgb(0xf0, 0x88, 0x3e),
        Hex::rgb(0xb8, 0xcc, 0x3a),
        Hex::rgb(0x1e, 0xd7, 0x60),
        Hex::rgb(0x46, 0xa9, 0xe0),
    ],
};
