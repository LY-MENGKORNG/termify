//! Lyrics, scrolled to the line being sung.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::Widget;

use crate::lyrics::Line;
use crate::utils::text;

/// Where in the area the current line sits, as a fraction from the top.
const ANCHOR: usize = 3;

/// Lyrics with the sung line picked out.
pub struct LyricsPane<'a> {
    lines: &'a [Line],
    /// Index of the line being sung, when it is known.
    current: Option<usize>,
    sung: Style,
    upcoming: Style,
    past: Style,
}

impl<'a> LyricsPane<'a> {
    /// Draws `lines`, highlighting `current`.
    #[must_use]
    pub const fn new(
        lines: &'a [Line],
        current: Option<usize>,
        sung: Style,
        upcoming: Style,
        past: Style,
    ) -> Self {
        Self {
            lines,
            current,
            sung,
            upcoming,
            past,
        }
    }

    /// Index of the first line to draw.
    fn first_visible(&self, height: usize) -> usize {
        let Some(current) = self.current else {
            return 0;
        };

        let anchor = (height / ANCHOR).max(1);
        let last_start = self.lines.len().saturating_sub(height);

        current.saturating_sub(anchor).min(last_start)
    }
}

impl Widget for LyricsPane<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.lines.is_empty() {
            return;
        }

        let height = usize::from(area.height);
        let width = usize::from(area.width);
        let first = self.first_visible(height);

        let drawn: Vec<TextLine> = self
            .lines
            .iter()
            .enumerate()
            .skip(first)
            .take(height)
            .map(|(index, line)| {
                let style = match self.current {
                    Some(current) if index == current => self.sung,
                    Some(current) if index < current => self.past,
                    // Unsynced lyrics have no "past": nothing has been sung yet
                    // as far as anything here knows.
                    _ => self.upcoming,
                };

                TextLine::from(Span::styled(
                    text::truncate(&line.text, width).into_owned(),
                    style,
                ))
            })
            .collect();

        super::text::render_lines(&drawn, area, buffer);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Style;
    use ratatui::widgets::Widget;

    use super::LyricsPane;
    use crate::lyrics::Line;

    /// `count` numbered lines.
    fn lines(count: usize) -> Vec<Line> {
        (0..count)
            .map(|index| Line {
                at: Duration::from_secs(index as u64),
                text: format!("Line {index}"),
            })
            .collect()
    }

    /// Draws and returns the rows as trimmed strings.
    fn draw(lines: &[Line], current: Option<usize>, width: u16, height: u16) -> Vec<String> {
        let area = Rect::new(0, 0, width, height);
        let mut buffer = Buffer::empty(area);

        LyricsPane::new(
            lines,
            current,
            Style::default(),
            Style::default(),
            Style::default(),
        )
        .render(area, &mut buffer);

        (0..height)
            .map(|row| {
                (0..width)
                    .map(|column| {
                        buffer
                            .cell((column, row))
                            .map_or(' ', |cell| cell.symbol().chars().next().unwrap_or(' '))
                    })
                    .collect::<String>()
                    .trim_end()
                    .to_owned()
            })
            .collect()
    }

    #[test]
    fn the_sung_line_sits_at_a_fixed_place_as_the_song_moves() {
        let lines = lines(60);

        // Two different positions mid-song: the current line is on the same row
        // both times, which is what stops the eye having to hunt for it.
        let early = draw(&lines, Some(20), 20, 9);
        let later = draw(&lines, Some(30), 20, 9);

        let row_of = |rows: &[String], text: &str| rows.iter().position(|row| row == text);

        assert_eq!(row_of(&early, "Line 20"), row_of(&later, "Line 30"));
    }

    #[test]
    fn the_start_of_a_song_is_not_scrolled_past() {
        let rows = draw(&lines(60), Some(0), 20, 6);

        assert_eq!(rows.first().map(String::as_str), Some("Line 0"));
    }

    #[test]
    fn the_end_of_a_song_fills_the_pane_rather_than_scrolling_into_blankness() {
        let rows = draw(&lines(20), Some(19), 20, 6);

        // The last line is the last row: no empty space below it.
        assert_eq!(rows.last().map(String::as_str), Some("Line 19"));
        assert!(rows.iter().all(|row| !row.is_empty()), "{rows:?}");
    }

    #[test]
    fn unsynced_lyrics_start_at_the_top() {
        let rows = draw(&lines(60), None, 20, 5);

        assert_eq!(rows.first().map(String::as_str), Some("Line 0"));
    }

    #[test]
    fn fewer_lines_than_rows_is_not_a_crash() {
        let rows = draw(&lines(2), Some(1), 20, 8);

        assert_eq!(rows.first().map(String::as_str), Some("Line 0"));
    }

    #[test]
    fn a_zero_sized_area_draws_nothing() {
        assert!(draw(&lines(4), Some(1), 0, 0).is_empty());
    }

    #[test]
    fn long_lines_are_truncated_to_the_width() {
        let long = vec![Line {
            at: Duration::ZERO,
            text: "A line far wider than the pane it has to fit inside".to_owned(),
        }];

        let rows = draw(&long, Some(0), 12, 1);

        assert_eq!(rows.first().map(|row| row.chars().count()), Some(12));
    }
}
