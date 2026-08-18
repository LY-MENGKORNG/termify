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

#[cfg(test)]
mod tests {
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::widgets::Widget;

    use crate::theme::Theme;

    use super::{Mirror, height_of, lit};

    /// Draws `levels` into a fresh buffer and returns it as lines.
    fn draw(levels: &[f32], width: u16, height: u16) -> Vec<String> {
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);
        let theme = Theme::dark();

        Mirror::new(levels, &theme).render(area, &mut buffer);

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

    /// Which rows have anything drawn in them.
    fn filled(lines: &[String]) -> Vec<usize> {
        lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.chars().any(|ch| ch != ' ' && ch != '\u{2800}'))
            .map(|(row, _)| row)
            .collect()
    }

    #[test]
    fn silence_is_a_line_through_the_middle() {
        // The one row holding the axis, and no other: an empty spectrum has to
        // look deliberate rather than broken.
        let lines = draw(&[0.0; 8], 12, 6);

        assert_eq!(filled(&lines), vec![3], "{lines:?}");
    }

    #[test]
    fn a_loud_spectrum_reaches_both_edges() {
        let lines = draw(&[1.0; 8], 12, 6);

        assert_eq!(filled(&lines), vec![0, 1, 2, 3, 4, 5], "{lines:?}");
    }

    #[test]
    fn louder_covers_more_rows_than_quieter() {
        let quiet = filled(&draw(&[0.2; 8], 12, 6)).len();
        let loud = filled(&draw(&[0.8; 8], 12, 6)).len();

        assert!(loud > quiet, "{loud} rows should beat {quiet}");
    }

    #[test]
    fn a_bar_covers_the_same_distance_above_and_below_the_axis() {
        // The mirroring, stated as the geometry rather than as glyphs: braille
        // bits do not read symmetrically even when the dots do.
        let (axis, reach) = (12, 5);

        for offset in 0..=reach {
            assert!(lit(axis - offset, axis, reach), "-{offset}");
            assert!(lit(axis + offset, axis, reach), "+{offset}");
        }

        assert!(!lit(axis - reach - 1, axis, reach));
        assert!(!lit(axis + reach + 1, axis, reach));
    }

    #[test]
    fn only_dots_are_drawn() {
        let lines = draw(&[0.5; 8], 12, 6);

        for line in &lines {
            for ch in line.chars() {
                assert!(
                    ch == ' ' || ('\u{2800}'..='\u{28ff}').contains(&ch),
                    "{ch:?} is not a braille cell"
                );
            }
        }
    }

    #[test]
    fn every_row_takes_its_colour_from_the_theme_at_its_own_height() {
        let area = Rect::new(0, 0, 4, 6);
        let mut buffer = Buffer::empty(area);
        let theme = Theme::dark();

        Mirror::new(&[1.0; 8], &theme).render(area, &mut buffer);

        let top = buffer.cell((0, 0)).map(|cell| cell.style().fg);
        let bottom = buffer.cell((0, 5)).map(|cell| cell.style().fg);

        assert_eq!(top, Some(theme.spectrum_at(height_of(0, 6)).fg));
        assert_eq!(bottom, Some(theme.spectrum_at(height_of(5, 6)).fg));
        assert_ne!(top, bottom, "the gradient has to go somewhere");
    }

    #[test]
    fn the_ramp_is_read_upwards() {
        // Height, not row index: the bottom row takes the first stop of the ramp
        // and the top row the last, whatever the area's height turns out to be.
        assert!(height_of(0, 6) > height_of(5, 6));
        assert!(height_of(0, 20) > height_of(19, 20));
    }

    #[test]
    fn nothing_is_drawn_outside_the_area() {
        let lines = draw(&[1.0; 40], 4, 2);

        assert_eq!(lines.len(), 2);
        // Counted in characters, not bytes: a braille cell is three bytes long.
        assert!(
            lines.iter().all(|line| line.chars().count() == 4),
            "{lines:?}"
        );
    }

    #[test]
    fn a_zero_sized_area_is_not_a_crash() {
        assert!(draw(&[1.0], 0, 0).is_empty());
    }

    #[test]
    fn no_levels_draws_nothing() {
        let lines = draw(&[], 8, 4);

        assert!(filled(&lines).is_empty(), "{lines:?}");
    }
}
