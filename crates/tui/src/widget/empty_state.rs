//! The "nothing here" state.

use ratatui::buffer::Buffer;
use ratatui::layout::{Alignment, Rect};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Widget};

use crate::theme::Theme;

/// A centred message with an optional hint line.
pub struct EmptyState<'a> {
    title: &'a str,
    hint: Option<Line<'a>>,
    marker: Option<&'a str>,
    theme: &'a Theme,
}

impl<'a> EmptyState<'a> {
    /// Creates an empty state with the given headline.
    #[must_use]
    pub const fn new(title: &'a str, theme: &'a Theme) -> Self {
        Self {
            title,
            hint: None,
            marker: None,
            theme,
        }
    }

    /// Adds a line below the headline, typically naming a key to press.
    #[must_use]
    pub fn hint(mut self, hint: Line<'a>) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Adds a glyph above the headline, e.g. a spinner frame.
    #[must_use]
    pub const fn marker(mut self, marker: &'a str) -> Self {
        self.marker = Some(marker);
        self
    }
}

impl Widget for EmptyState<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.height == 0 || area.width == 0 {
            return;
        }

        let mut lines = Vec::with_capacity(4);

        if let Some(marker) = self.marker {
            lines.push(Line::from(Span::styled(marker, self.theme.accent())));
            lines.push(Line::default());
        }

        lines.push(Line::from(Span::styled(self.title, self.theme.title())));

        if let Some(hint) = self.hint {
            lines.push(Line::default());
            lines.push(hint);
        }

        // Vertically centre by padding above, rather than by computing a
        // sub-rect: it keeps the text block intact if the area is short.
        let content_height = lines.len() as u16;
        let padding = area.height.saturating_sub(content_height) / 2;

        let mut text = Vec::with_capacity(lines.len() + usize::from(padding));
        text.extend(std::iter::repeat_n(Line::default(), usize::from(padding)));
        text.extend(lines);

        Paragraph::new(Text::from(text))
            .alignment(Alignment::Center)
            .render(area, buffer);
    }
}
