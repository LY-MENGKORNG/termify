//! The progress bar.

use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// A wholly elapsed cell.
const FILLED: &str = "━";

/// A half-elapsed cell, for sub-cell precision at the leading edge.
const HALF: &str = "╸";

/// A cell yet to elapse. Lighter weight than [`FILLED`], so the boundary reads
/// clearly even in a monochrome terminal.
const EMPTY: &str = "─";

/// Renders a one-row progress bar exactly `width` columns wide.
#[must_use]
pub fn bar(ratio: f64, width: u16, theme: &Theme) -> Line<'static> {
    let width = usize::from(width);
    if width == 0 {
        return Line::default();
    }

    let exact = ratio.clamp(0.0, 1.0) * width as f64;
    let full = (exact.floor() as usize).min(width);
    let half = full < width && (exact - full as f64) >= 0.5;
    let empty = width - full - usize::from(half);

    let mut spans = Vec::with_capacity(3);

    if full > 0 {
        spans.push(Span::styled(FILLED.repeat(full), theme.progress_filled()));
    }
    if half {
        spans.push(Span::styled(HALF, theme.progress_filled()));
    }
    if empty > 0 {
        spans.push(Span::styled(EMPTY.repeat(empty), theme.progress_empty()));
    }

    Line::from(spans)
}
