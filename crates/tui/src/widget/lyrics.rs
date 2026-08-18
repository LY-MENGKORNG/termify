//! Lyrics, scrolled to the line being sung.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::Widget;

use termify_core::constant::ANCHOR;
use termify_core::service::lyric::lrc::Line;
use termify_core::util::text;

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
