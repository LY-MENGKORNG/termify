use crate::services::theme::{dark, ember, midnight, neon, palette::Palette, paper};

/// Stops in the spectrum ramp, bottom of the gradient first.
///
/// Six is enough to carry a full hue sweep and few enough that writing one by
/// hand in a theme file is not a chore.
pub const SPECTRUM_STOPS: usize = 6;

/// Every theme compiled into the binary, in the order the picker lists them.
///
/// A file in `themes/` may reuse one of these names, in which case the file
/// wins — that is how the default palette gets retuned without forking it.
pub const BUILT_IN: [(&str, Palette); 5] = [
    (dark::NAME, dark::PALETTE),
    (midnight::NAME, midnight::PALETTE),
    (neon::NAME, neon::PALETTE),
    (ember::NAME, ember::PALETTE),
    (paper::NAME, paper::PALETTE),
];
