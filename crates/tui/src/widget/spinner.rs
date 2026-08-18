//! A subtle activity indicator.

/// Braille frames: single-width, and they animate without shifting weight
/// around, so the motion reads as calm rather than flickering.
const FRAMES: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// The frame to draw for the given animation counter.
#[must_use]
pub fn frame(counter: u64) -> &'static str {
    let index = (counter % FRAMES.len() as u64) as usize;
    FRAMES.get(index).copied().unwrap_or(FRAMES[0])
}
