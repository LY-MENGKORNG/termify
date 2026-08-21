//! The spectrum, drawn in block characters.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Widget;

/// The eight partial blocks, tallest last. Index zero is empty.
const STEPS: [&str; 9] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];

/// Vertical steps one cell can represent.
const PER_CELL: usize = 8;

/// Columns of gap between bars, so they read as bars rather than as a block.
const GAP: u16 = 1;

/// A row of vertical bars, each a level in `0.0..=1.0`.
pub struct Bars<'a> {
    levels: &'a [f32],
    style: Style,
}

impl<'a> Bars<'a> {
    /// Draws `levels` in `style`.
    #[must_use]
    pub const fn new(levels: &'a [f32], style: Style) -> Self {
        Self { levels, style }
    }
}

/// Where band `index` of `bands` starts and ends across `width` columns.
fn span(index: usize, bands: usize, width: u16) -> (u16, u16) {
    let bands = bands.max(1);
    let width = u32::from(width);

    let start = (width * index as u32 / bands as u32) as u16;
    let end = (width * (index as u32 + 1) / bands as u32) as u16;

    (start, end)
}

impl Widget for Bars<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.levels.is_empty() {
            return;
        }

        let steps = usize::from(area.height) * PER_CELL;
        let bands = self.levels.len();

        for (index, level) in self.levels.iter().enumerate() {
            let (start, end) = span(index, bands, area.width);
            // A gap only when there is room for one; at a column per band the
            // bars run together rather than vanishing.
            let width = end.saturating_sub(start).saturating_sub(GAP).max(1);
            if start >= area.width {
                break;
            }

            let filled = (level.clamp(0.0, 1.0) * steps as f32).round() as usize;
            let x = area.x + start;
            let width = width.min(area.width - start);

            for row in 0..area.height {
                // Rows are drawn top-down; bars grow bottom-up.
                let from_bottom = usize::from(area.height - row - 1);
                let cell_steps = filled.saturating_sub(from_bottom * PER_CELL).min(PER_CELL);

                let Some(glyph) = STEPS.get(cell_steps) else {
                    continue;
                };
                if cell_steps == 0 {
                    continue;
                }

                for column in 0..width {
                    if let Some(cell) = buffer.cell_mut((x + column, area.y + row)) {
                        cell.set_symbol(glyph).set_style(self.style);
                    }
                }
            }
        }
    }
}
