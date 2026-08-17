//! Keyboard hints.

use ratatui::text::Span;

use crate::events::Action;
use crate::input::{Chord, KeyMap};
use crate::theme::Theme;

/// Separator between hints. A middle dot rather than a pipe: quieter.
pub const SEPARATOR: &str = " · ";

/// Renders `key label` as two spans, the key emphasised.
#[must_use]
pub fn chip<'a>(chord: &Chord, label: &'a str, theme: &Theme) -> [Span<'a>; 3] {
    [
        Span::styled(chord.to_string(), theme.key_hint()),
        Span::raw(" "),
        Span::styled(label, theme.muted()),
    ]
}

/// Renders a hint for `action`, or nothing when it is unbound.
#[must_use]
pub fn for_action<'a>(
    keymap: &KeyMap,
    action: Action,
    label: &'a str,
    theme: &Theme,
) -> Vec<Span<'a>> {
    match keymap.chord_for(action) {
        Some(chord) => chip(&chord, label, theme).to_vec(),
        None => Vec::new(),
    }
}

/// Joins hints with [`SEPARATOR`], skipping any that are empty.
#[must_use]
pub fn join<'a>(groups: Vec<Vec<Span<'a>>>, theme: &Theme) -> Vec<Span<'a>> {
    let mut spans = Vec::new();

    for group in groups {
        if group.is_empty() {
            continue;
        }
        if !spans.is_empty() {
            spans.push(Span::styled(SEPARATOR, theme.muted()));
        }
        spans.extend(group);
    }

    spans
}
