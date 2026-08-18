//! Width-aware text fitting.

use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthChar;

/// Character appended when text is cut short.
const ELLIPSIS: char = '…';

/// Gap inserted between repetitions of scrolling text.
const MARQUEE_GAP: &str = "   ";

/// Forces emoji presentation, and with it a double-width cell.
const VARIATION_SELECTOR_16: char = '\u{FE0F}';

/// Splits `text` into grapheme clusters.
#[must_use]
pub fn clusters(text: &str) -> Vec<&str> {
    text.graphemes(true).collect()
}

/// Width of one grapheme cluster in terminal cells.
#[must_use]
pub fn cluster_width(cluster: &str) -> usize {
    let mut chars = cluster.chars();
    let Some(base) = chars.next() else {
        return 0;
    };

    // A flag is two regional indicators and renders as one double-width glyph.
    if is_regional_indicator(base) && chars.clone().next().is_some_and(is_regional_indicator) {
        return 2;
    }

    if cluster.contains(VARIATION_SELECTOR_16) {
        return 2;
    }

    UnicodeWidthChar::width(base).unwrap_or(0)
}

/// Whether `ch` is one half of a flag sequence.
fn is_regional_indicator(ch: char) -> bool {
    ('\u{1F1E6}'..='\u{1F1FF}').contains(&ch)
}

/// Width of `text` in terminal cells.
#[must_use]
pub fn display_width(text: &str) -> usize {
    text.graphemes(true).map(cluster_width).sum()
}

/// Truncates `text` to `max_width` cells, ending with an ellipsis.
#[must_use]
pub fn truncate(text: &str, max_width: usize) -> Cow<'_, str> {
    if display_width(text) <= max_width {
        return Cow::Borrowed(text);
    }

    if max_width == 0 {
        return Cow::Borrowed("");
    }

    // Reserve one cell for the ellipsis itself.
    let budget = max_width.saturating_sub(1);
    let mut out = String::with_capacity(text.len().min(max_width * 4));
    let mut width = 0;

    for cluster in text.graphemes(true) {
        let cluster_width = cluster_width(cluster);
        if width + cluster_width > budget {
            break;
        }
        out.push_str(cluster);
        width += cluster_width;
    }

    out.push(ELLIPSIS);
    Cow::Owned(out)
}

/// Pads `text` with spaces to exactly `width` cells, truncating if needed.
#[must_use]
pub fn fit(text: &str, width: usize) -> Cow<'_, str> {
    let current = display_width(text);

    if current == width {
        return Cow::Borrowed(text);
    }

    // Truncation can land short of the budget when the next cluster is wider
    // than the cells left over, so the padding below runs either way.
    let (text, current) = if current > width {
        let cut = truncate(text, width).into_owned();
        let cut_width = display_width(&cut);
        if cut_width == width {
            return Cow::Owned(cut);
        }
        (Cow::Owned(cut), cut_width)
    } else {
        (Cow::Borrowed(text), current)
    };

    let mut out = String::with_capacity(text.len() + (width - current));
    out.push_str(&text);
    out.extend(std::iter::repeat_n(' ', width - current));
    Cow::Owned(out)
}

/// A scrolling window over text too long to fit.
#[must_use]
pub fn marquee(text: &str, offset: usize, width: usize) -> Cow<'_, str> {
    if display_width(text) <= width {
        return Cow::Borrowed(text);
    }

    if width == 0 {
        return Cow::Borrowed("");
    }

    let mut cycle = clusters(text);
    cycle.extend(clusters(MARQUEE_GAP));
    if cycle.is_empty() {
        return Cow::Borrowed("");
    }

    let start = offset % cycle.len();
    let mut out = String::with_capacity(width * 4);
    let mut filled = 0;

    for step in 0..cycle.len() {
        let cluster = cycle
            .get((start + step) % cycle.len())
            .copied()
            .unwrap_or(" ");
        let cluster_width = cluster_width(cluster);
        if filled + cluster_width > width {
            break;
        }
        out.push_str(cluster);
        filled += cluster_width;
    }

    Cow::Owned(out)
}

/// Number of marquee steps before the animation repeats.
#[must_use]
pub fn marquee_period(text: &str) -> usize {
    clusters(text).len() + clusters(MARQUEE_GAP).len()
}
