//! Drawing text at the width the terminal actually uses.

use std::num::NonZeroU16;

use ratatui::buffer::{Buffer, CellDiffOption};
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Style;
use ratatui::text::Line;

use termify_core::util::text;

/// Draws `lines` from the top of `area`, one per row.
pub fn render_lines(lines: &[Line<'_>], area: Rect, buffer: &mut Buffer) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    for (row, line) in lines.iter().take(area.height as usize).enumerate() {
        let Ok(row) = u16::try_from(row) else {
            break;
        };
        let Some(y) = area.y.checked_add(row) else {
            break;
        };

        render_at(line, area, y, buffer);
    }
}

/// Draws a single line at the top of `area`.
pub fn render_line(line: &Line<'_>, area: Rect, buffer: &mut Buffer) {
    render_lines(std::slice::from_ref(line), area, buffer);
}

/// Draws one line on row `y`, honouring its alignment.
fn render_at(line: &Line<'_>, area: Rect, y: u16, buffer: &mut Buffer) {
    let width = line_width(line);
    let mut x = area
        .x
        .saturating_add(offset(line.alignment, width, area.width));

    for span in &line.spans {
        let style = line.style.patch(span.style);
        x = render_span(&span.content, style, x, y, area, buffer);
    }
}

/// Writes one span's clusters, returning where the next span starts.
fn render_span(
    content: &str,
    style: Style,
    mut x: u16,
    y: u16,
    area: Rect,
    buf: &mut Buffer,
) -> u16 {
    let right = area.x.saturating_add(area.width);

    for cluster in content.graphemes() {
        let width = text::cluster_width(cluster);

        // Zero-width clusters have no cell of their own; dropping them keeps
        // them from overwriting the glyph they belong to.
        let Some(forced) = u16::try_from(width).ok().and_then(NonZeroU16::new) else {
            continue;
        };

        if x >= right || x.saturating_add(forced.get()) > right {
            break;
        }

        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_symbol(cluster)
                .set_style(style)
                .set_diff_option(CellDiffOption::ForcedWidth(forced));
        }

        // Cells the glyph covers hold nothing of their own, or the terminal
        // would be told to draw over the tail of the character before them.
        for trailing in 1..forced.get() {
            if let Some(cell) = buf.cell_mut((x.saturating_add(trailing), y)) {
                cell.reset();
                cell.set_style(style);
            }
        }

        x = x.saturating_add(forced.get());
    }

    x
}

/// Total width of a line in cells.
fn line_width(line: &Line<'_>) -> u16 {
    let total: usize = line
        .spans
        .iter()
        .map(|span| text::display_width(&span.content))
        .sum();

    u16::try_from(total).unwrap_or(u16::MAX)
}

/// Leading cells for an alignment.
fn offset(alignment: Option<Alignment>, width: u16, available: u16) -> u16 {
    let slack = available.saturating_sub(width);

    match alignment {
        Some(Alignment::Center) => slack / 2,
        Some(Alignment::Right) => slack,
        _ => 0,
    }
}

/// Grapheme iteration, kept local so callers need not import the trait.
trait Graphemes {
    /// Splits into grapheme clusters.
    fn graphemes(&self) -> unicode_segmentation::Graphemes<'_>;
}

impl Graphemes for str {
    fn graphemes(&self) -> unicode_segmentation::Graphemes<'_> {
        unicode_segmentation::UnicodeSegmentation::graphemes(self, true)
    }
}
