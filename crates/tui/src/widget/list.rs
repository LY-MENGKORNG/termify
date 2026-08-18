//! Scroll-window arithmetic for lists.

use std::ops::Range;

/// The slice of rows to draw so that `selected` is visible.
#[must_use]
pub fn window(selected: Option<usize>, len: usize, height: u16) -> Range<usize> {
    let height = usize::from(height);

    if height == 0 || len == 0 {
        return 0..0;
    }

    if len <= height {
        return 0..len;
    }

    let selected = selected.unwrap_or(0).min(len - 1);

    // Centre the selection when possible, then clamp to the ends so the first
    // and last screens are full rather than half-empty.
    let half = height / 2;
    let start = selected.saturating_sub(half).min(len - height);

    start..start + height
}
