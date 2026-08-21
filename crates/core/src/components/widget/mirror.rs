//! The spectrum as a mirrored field of dots.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

use crate::theme::Theme;

/// Dot rows in one braille cell.
const DOT_ROWS: u16 = 4;

/// Dot columns in one braille cell.
const DOT_COLUMNS: u16 = 2;

/// The empty braille cell. Dots are added to it as bits.
const BRAILLE: u32 = 0x2800;

/// Bit for each dot of the left column, topmost first.
const LEFT: [u8; 4] = [0x01, 0x02, 0x04, 0x40];

/// Bit for each dot of the right column, topmost first.
const RIGHT: [u8; 4] = [0x08, 0x10, 0x20, 0x80];

/// A mirrored spectrum, each level in `0.0..=1.0`.
pub struct Mirror<'a> {
    levels: &'a [f32],
    theme: &'a Theme,
}

impl<'a> Mirror<'a> {
    /// Draws `levels` in `theme`'s spectrum gradient.
    #[must_use]
    pub const fn new(levels: &'a [f32], theme: &'a Theme) -> Self {
        Self { levels, theme }
    }
}

impl Widget for Mirror<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.levels.is_empty() {
            return;
        }

        let dot_rows = area.height.saturating_mul(DOT_ROWS);
        let dot_columns = area.width.saturating_mul(DOT_COLUMNS);

        // How far a bar may travel either side of the axis — the smaller of the
        // two distances, so a loud passage cannot clip on one side only.
        let axis = dot_rows / 2;
        let travel = axis.min(dot_rows - 1 - axis);

        // A row at a time, so the gradient is sampled once per row rather than
        // once per cell.
        for row in 0..area.height {
            let style = self.theme.spectrum_at(height_of(row, area.height));

            for column in 0..area.width {
                let mask = dots(
                    self.levels,
                    row,
                    column,
                    Field {
                        axis,
                        travel,
                        dot_columns,
                    },
                );

                if mask == 0 {
                    continue;
                }

                let Some(symbol) = char::from_u32(BRAILLE + u32::from(mask)) else {
                    continue;
                };

                if let Some(cell) = buffer.cell_mut((area.x + column, area.y + row)) {
                    cell.set_char(symbol).set_style(style);
                }
            }
        }
    }
}

/// The geometry every cell is measured against.
#[derive(Debug, Clone, Copy)]
struct Field {
    /// Dot row the bars grow out of.
    axis: u16,
    /// Dot rows a full-height bar covers either side of the axis.
    travel: u16,
    /// Dot columns across the whole area.
    dot_columns: u16,
}

/// The dots lit inside one cell, as a braille bit mask.
fn dots(levels: &[f32], row: u16, column: u16, field: Field) -> u8 {
    let mut mask = 0;

    // Each half of the cell is its own column of the spectrum, which is where
    // the extra horizontal resolution comes from.
    for (side, bits) in [LEFT, RIGHT].into_iter().enumerate() {
        let dot_column = column
            .saturating_mul(DOT_COLUMNS)
            .saturating_add(side as u16);
        let level = level_at(levels, across(dot_column, field.dot_columns));
        let reach = reach_of(level, field.travel);

        for (dot, bit) in bits.into_iter().enumerate() {
            let dot_row = row.saturating_mul(DOT_ROWS).saturating_add(dot as u16);
            if lit(dot_row, field.axis, reach) {
                mask |= bit;
            }
        }
    }

    mask
}

/// How far up the area `row` sits: `0.0` at the bottom, `1.0` at the top.
fn height_of(row: u16, rows: u16) -> f32 {
    1.0 - (f32::from(row) + 0.5) / f32::from(rows.max(1))
}

/// Where a dot column sits across the width, in `0.0..=1.0`.
fn across(dot_column: u16, dot_columns: u16) -> f32 {
    let last = dot_columns.saturating_sub(1);
    if last == 0 {
        return 0.0;
    }

    f32::from(dot_column.min(last)) / f32::from(last)
}

/// The level at `position` across the bands, mixing neighbours.
fn level_at(levels: &[f32], position: f32) -> f32 {
    let Some(first) = levels.first().copied() else {
        return 0.0;
    };

    let last = levels.len().saturating_sub(1);
    if last == 0 {
        return first.clamp(0.0, 1.0);
    }

    let scaled = position.clamp(0.0, 1.0) * last as f32;
    let index = scaled.floor() as usize;
    let fraction = scaled - index as f32;

    let low = levels.get(index).copied().unwrap_or(first);
    let high = levels.get(index + 1).copied().unwrap_or(low);

    (low + (high - low) * fraction).clamp(0.0, 1.0)
}

/// Dot rows a level reaches either side of the axis.
fn reach_of(level: f32, travel: u16) -> u16 {
    (level.clamp(0.0, 1.0) * f32::from(travel)).round() as u16
}

/// Whether a dot row falls inside the band a bar covers.
fn lit(dot_row: u16, axis: u16, reach: u16) -> bool {
    dot_row >= axis.saturating_sub(reach) && dot_row <= axis.saturating_add(reach)
}
