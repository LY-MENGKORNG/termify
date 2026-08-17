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

#[cfg(test)]
mod tests {
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Style;
    use ratatui::widgets::Widget;

    use super::Bars;

    /// Draws `levels` into a fresh buffer and returns it as lines.
    fn draw(levels: &[f32], width: u16, height: u16) -> Vec<String> {
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);

        Bars::new(levels, Style::default()).render(area, &mut buffer);

        (0..height)
            .map(|row| {
                (0..width)
                    .map(|column| {
                        buffer
                            .cell((column, row))
                            .map_or(' ', |cell| cell.symbol().chars().next().unwrap_or(' '))
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn a_full_bar_fills_every_row() {
        let lines = draw(&[1.0], 2, 3);

        assert!(lines.iter().all(|line| line.starts_with('█')), "{lines:?}");
    }

    #[test]
    fn an_empty_bar_draws_nothing() {
        let lines = draw(&[0.0], 2, 3);

        assert!(lines.iter().all(|line| line.trim().is_empty()), "{lines:?}");
    }

    #[test]
    fn bars_grow_from_the_bottom() {
        // A third of three rows: the bottom row only.
        let lines = draw(&[0.33], 2, 3);

        assert!(lines.first().is_some_and(|line| line.trim().is_empty()));
        assert!(lines.last().is_some_and(|line| !line.trim().is_empty()));
    }

    #[test]
    fn a_partial_row_uses_a_partial_block() {
        // One row tall, half full: a middle block rather than a full one.
        let lines = draw(&[0.5], 1, 1);

        assert_eq!(lines.first().map(|line| line.starts_with('▄')), Some(true));
    }

    #[test]
    fn bars_span_the_whole_width() {
        // Twelve bands over thirty-six columns: the last bar must reach the
        // right-hand edge rather than stopping short of it.
        let lines = draw(&[1.0; 12], 36, 1);
        let last = lines
            .first()
            .and_then(|line| line.chars().rev().position(|ch| ch == '█'));

        // One column short of the edge: that column is the trailing gap.
        assert_eq!(last, Some(1), "{lines:?}");
    }

    #[test]
    fn more_bands_than_columns_draws_as_many_as_fit() {
        let lines = draw(&[1.0; 40], 4, 1);

        // Nothing written outside the area. Counted in characters, not bytes:
        // a block glyph is three bytes long.
        assert_eq!(lines.len(), 1);
        assert_eq!(lines.first().map(|line| line.chars().count()), Some(4));
    }

    #[test]
    fn a_zero_sized_area_is_not_a_crash() {
        assert!(draw(&[1.0], 0, 0).is_empty());
    }
}
