//! Section headings.

use ratatui::text::{Line, Span};

use crate::theme::Theme;

/// A heading, drawn as text rather than as a bordered box.
#[must_use]
pub fn heading<'a>(text: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(Span::styled(text, theme.heading()))
}

/// A quieter label, for metadata above a heading.
#[must_use]
pub fn label<'a>(text: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(Span::styled(text, theme.muted()))
}
