//! Key chords, and the two-key sequences that need a little memory.

use std::fmt;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// A single key together with its modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Chord {
    /// The key itself.
    pub code: KeyCode,
    /// Held modifiers, normalised.
    pub modifiers: KeyModifiers,
}

impl Chord {
    /// Builds a chord with no modifiers.
    #[must_use]
    pub const fn key(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    /// Builds a chord with explicit modifiers.
    #[must_use]
    pub const fn with(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    /// Normalises a terminal key event into a chord.
    #[must_use]
    pub fn from_event(event: KeyEvent) -> Self {
        let mut modifiers = event.modifiers;
        if matches!(event.code, KeyCode::Char(_)) {
            modifiers.remove(KeyModifiers::SHIFT);
        }
        Self {
            code: event.code,
            modifiers,
        }
    }

    /// Parses a binding written as `"g"`, `"ctrl+d"`, `"shift+tab"`, `"esc"`.
    pub fn parse(text: &str) -> Result<Self, ParseChordError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(ParseChordError::new(text));
        }

        let mut modifiers = KeyModifiers::NONE;
        let mut parts: Vec<&str> = text.split('+').collect();

        // A trailing `+` means the key *is* plus, e.g. "ctrl++".
        let key = if text.ends_with('+') && parts.len() > 1 {
            parts.pop();
            parts.pop();
            "+"
        } else {
            parts.pop().ok_or_else(|| ParseChordError::new(text))?
        };

        for part in parts {
            match part.trim().to_ascii_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= KeyModifiers::CONTROL,
                "alt" | "meta" | "option" => modifiers |= KeyModifiers::ALT,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                "super" | "cmd" | "command" => modifiers |= KeyModifiers::SUPER,
                _ => return Err(ParseChordError::new(text)),
            }
        }

        let code = parse_code(key).ok_or_else(|| ParseChordError::new(text))?;

        // Keep normalisation identical to `from_event`, or a parsed binding
        // would never match a real key press.
        if matches!(code, KeyCode::Char(_)) {
            modifiers.remove(KeyModifiers::SHIFT);
        }

        Ok(Self { code, modifiers })
    }
}

fn parse_code(key: &str) -> Option<KeyCode> {
    let mut chars = key.chars();
    if let (Some(ch), None) = (chars.next(), chars.next()) {
        return Some(KeyCode::Char(ch));
    }

    let code = match key.to_ascii_lowercase().as_str() {
        "space" => KeyCode::Char(' '),
        "enter" | "return" | "cr" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "esc" | "escape" => KeyCode::Esc,
        "backspace" | "bs" => KeyCode::Backspace,
        "delete" | "del" => KeyCode::Delete,
        "insert" => KeyCode::Insert,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" | "pgup" => KeyCode::PageUp,
        "pagedown" | "pgdn" => KeyCode::PageDown,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        other => {
            let number = other.strip_prefix('f')?.parse().ok()?;
            KeyCode::F(number)
        }
    };
    Some(code)
}

impl fmt::Display for Chord {
    /// Renders the chord the way the help overlay shows it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            f.write_str("ctrl+")?;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            f.write_str("alt+")?;
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            f.write_str("shift+")?;
        }

        match self.code {
            KeyCode::Char(' ') => f.write_str("space"),
            KeyCode::Char(ch) => write!(f, "{ch}"),
            KeyCode::Enter => f.write_str("enter"),
            KeyCode::Tab => f.write_str("tab"),
            KeyCode::BackTab => f.write_str("shift+tab"),
            KeyCode::Esc => f.write_str("esc"),
            KeyCode::Backspace => f.write_str("backspace"),
            KeyCode::Up => f.write_str("↑"),
            KeyCode::Down => f.write_str("↓"),
            KeyCode::Left => f.write_str("←"),
            KeyCode::Right => f.write_str("→"),
            KeyCode::F(number) => write!(f, "f{number}"),
            other => write!(f, "{other:?}"),
        }
    }
}

/// A binding string could not be parsed.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("`{text}` is not a key binding; try \"g\", \"ctrl+d\", \"shift+tab\", or \"esc\"")]
pub struct ParseChordError {
    /// The rejected text.
    pub text: String,
}

impl ParseChordError {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_owned(),
        }
    }
}

/// The half-typed state of a two-key sequence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Pending {
    /// Nothing typed.
    #[default]
    Empty,
    /// `g` was pressed; a second `g` completes the sequence.
    G,
}

impl Pending {
    /// Whether a sequence is half-typed, which the status line indicates.
    #[must_use]
    pub const fn is_active(self) -> bool {
        !matches!(self, Self::Empty)
    }
}
