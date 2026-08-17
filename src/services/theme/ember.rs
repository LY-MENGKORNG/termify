//! A warm built-in theme, for rooms with the lights off.
//!
//! Every surface is tinted towards brown rather than blue, which is the whole
//! point: at night a warm screen is easier to sit in front of for an album.

use super::color::Hex;
use super::palette::Palette;

/// Name reported by this theme.
pub const NAME: &str = "ember";

/// Brown surfaces, an amber accent.
pub const PALETTE: Palette = Palette {
    bg: Hex::rgb(0x14, 0x10, 0x0e),
    surface: Hex::rgb(0x1c, 0x16, 0x13),
    overlay: Hex::rgb(0x24, 0x1d, 0x18),
    border: Hex::rgb(0x35, 0x2b, 0x23),
    selection: Hex::rgb(0x2c, 0x22, 0x1b),

    text: Hex::rgb(0xf2, 0xe8, 0xdd),
    subtext: Hex::rgb(0xc2, 0xad, 0x9a),
    muted: Hex::rgb(0x87, 0x76, 0x68),

    accent: Hex::rgb(0xff, 0x8a, 0x4c),
    accent_alt: Hex::rgb(0xf2, 0xc1, 0x4e),

    success: Hex::rgb(0xa8, 0xc4, 0x6c),
    warning: Hex::rgb(0xf2, 0xc1, 0x4e),
    error: Hex::rgb(0xf2, 0x66, 0x5e),

    progress_filled: Hex::rgb(0xff, 0x8a, 0x4c),
    progress_empty: Hex::rgb(0x35, 0x2b, 0x23),

    spectrum: [
        Hex::rgb(0xc1, 0x35, 0x5f),
        Hex::rgb(0xec, 0x5f, 0x4a),
        Hex::rgb(0xff, 0x8a, 0x4c),
        Hex::rgb(0xf2, 0xc1, 0x4e),
        Hex::rgb(0xd2, 0xd0, 0x6a),
        Hex::rgb(0xa8, 0xc4, 0x6c),
    ],
};
