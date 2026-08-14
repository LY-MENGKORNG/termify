//! A loud built-in theme, for the spectrum.
//!
//! The one palette here that is not quiet. Surfaces stay nearly black so the
//! saturated foregrounds have somewhere to glow, and the gradient is a full hue
//! sweep rather than a warm-to-cool lean — the mirrored spectrum is the reason
//! this theme exists.

use super::color::Hex;
use super::palette::Palette;

/// Name reported by this theme.
pub const NAME: &str = "neon";

/// Near-black surfaces, magenta accent, cyan second.
pub const PALETTE: Palette = Palette {
    bg: Hex::rgb(0x08, 0x06, 0x0f),
    surface: Hex::rgb(0x10, 0x0c, 0x1c),
    overlay: Hex::rgb(0x17, 0x10, 0x29),
    border: Hex::rgb(0x2a, 0x1f, 0x45),
    selection: Hex::rgb(0x24, 0x1a, 0x3d),

    text: Hex::rgb(0xf2, 0xec, 0xff),
    subtext: Hex::rgb(0xb6, 0xa8, 0xdd),
    muted: Hex::rgb(0x7a, 0x6b, 0xa3),

    accent: Hex::rgb(0xff, 0x2f, 0xb9),
    accent_alt: Hex::rgb(0x23, 0xe0, 0xff),

    success: Hex::rgb(0x39, 0xff, 0x88),
    warning: Hex::rgb(0xff, 0xd5, 0x4a),
    error: Hex::rgb(0xff, 0x4f, 0x6e),

    progress_filled: Hex::rgb(0xff, 0x2f, 0xb9),
    progress_empty: Hex::rgb(0x2a, 0x1f, 0x45),

    spectrum: [
        Hex::rgb(0xff, 0x2f, 0xb9),
        Hex::rgb(0xc0, 0x4d, 0xff),
        Hex::rgb(0x4f, 0x7c, 0xff),
        Hex::rgb(0x23, 0xe0, 0xff),
        Hex::rgb(0x39, 0xff, 0x88),
        Hex::rgb(0xea, 0xff, 0x5c),
    ],
};
