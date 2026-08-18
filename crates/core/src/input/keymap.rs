//! The key-to-action table.

use std::collections::BTreeMap;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::{event::Action, state::route::Route};

use super::{Chord, Mode, Pending};

/// What a key press resolved to, and what the pending sequence becomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolved {
    /// The action to dispatch, if the press completed one.
    pub action: Option<Action>,
    /// The sequence state to carry into the next press.
    pub pending: Pending,
}

impl Resolved {
    /// Nothing happened; any half-typed sequence is abandoned.
    const NOTHING: Self = Self {
        action: None,
        pending: Pending::Empty,
    };

    const fn action(action: Action) -> Self {
        Self {
            action: Some(action),
            pending: Pending::Empty,
        }
    }

    const fn pending(pending: Pending) -> Self {
        Self {
            action: None,
            pending,
        }
    }
}

/// A problem found while applying user overrides.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BindingWarning {
    /// The key could not be parsed.
    #[error("{0}")]
    BadKey(#[from] super::ParseChordError),
    /// The action name is not recognised.
    #[error("`{name}` is not an action; press ? to see the available actions")]
    UnknownAction {
        /// The rejected name.
        name: String,
    },
}

/// Bindings for every mode.
#[derive(Debug, Clone)]
pub struct KeyMap {
    normal: Vec<(Chord, Action)>,
    modal: Vec<(Chord, Action)>,
    palette: Vec<(Chord, Action)>,
    search: Vec<(Chord, Action)>,
}

impl KeyMap {
    /// Resolves a key press in the given mode.
    #[must_use]
    pub fn resolve(&self, mode: Mode, chord: Chord, pending: Pending) -> Resolved {
        match mode {
            Mode::Palette => Self::resolve_typing(&self.palette, chord, Action::PaletteInsert),
            Mode::Search => Self::resolve_typing(&self.search, chord, Action::SearchInsert),
            Mode::Normal => Self::resolve_sequence(&self.normal, chord, pending),
            Mode::Modal => Self::resolve_sequence(&self.modal, chord, pending),
        }
    }

    /// Bindings for a mode, in declaration order.
    pub fn bindings(&self, mode: Mode) -> impl Iterator<Item = (Chord, Action)> {
        let table = match mode {
            Mode::Normal => &self.normal,
            Mode::Modal => &self.modal,
            Mode::Palette => &self.palette,
            Mode::Search => &self.search,
        };
        table.iter().copied()
    }

    /// The primary key bound to `action`, for on-screen hints.
    #[must_use]
    pub fn chord_for(&self, action: Action) -> Option<Chord> {
        self.normal
            .iter()
            .find(|(_, bound)| *bound == action)
            .map(|(chord, _)| *chord)
    }

    /// Applies `[keys]` overrides from the configuration to normal mode.
    pub fn apply_overrides(&mut self, overrides: &BTreeMap<String, String>) -> Vec<BindingWarning> {
        let mut warnings = Vec::new();

        for (key, action_name) in overrides {
            let chord = match Chord::parse(key) {
                Ok(chord) => chord,
                Err(error) => {
                    warnings.push(BindingWarning::BadKey(error));
                    continue;
                }
            };

            let Some(action) = Action::from_name(action_name) else {
                warnings.push(BindingWarning::UnknownAction {
                    name: action_name.clone(),
                });
                continue;
            };

            match self.normal.iter_mut().find(|(bound, _)| *bound == chord) {
                Some(entry) => entry.1 = action,
                None => self.normal.push((chord, action)),
            }
        }

        warnings
    }

    /// Handles the one two-key sequence, then falls back to a direct lookup.
    fn resolve_sequence(table: &[(Chord, Action)], chord: Chord, pending: Pending) -> Resolved {
        const G: Chord = Chord::key(KeyCode::Char('g'));

        if chord == G {
            return match pending {
                // The second `g` completes the sequence.
                Pending::G => Resolved::action(Action::GoToTop),
                Pending::Empty => Resolved::pending(Pending::G),
            };
        }

        // Any other key abandons a half-typed sequence and is handled normally.
        table
            .iter()
            .find(|(bound, _)| *bound == chord)
            .map_or(Resolved::NOTHING, |(_, action)| Resolved::action(*action))
    }

    /// Where text is being typed, printable keys are input and the rest are
    /// commands. Shared by the palette and the search box, which differ only in
    /// which action carries the character.
    fn resolve_typing(
        table: &[(Chord, Action)],
        chord: Chord,
        insert: fn(char) -> Action,
    ) -> Resolved {
        if let Some((_, action)) = table.iter().find(|(bound, _)| *bound == chord) {
            return Resolved::action(*action);
        }

        // Only unmodified printable characters become input, so `ctrl+w` does
        // not silently type a `w`.
        let typing = chord.modifiers.is_empty() || chord.modifiers == KeyModifiers::SHIFT;

        match chord.code {
            KeyCode::Char(ch) if typing => Resolved::action(insert(ch)),
            _ => Resolved::NOTHING,
        }
    }
}

impl Default for KeyMap {
    /// The documented default bindings.
    fn default() -> Self {
        use Action as A;
        use KeyCode as K;

        let ctrl = |code: KeyCode| Chord::with(code, KeyModifiers::CONTROL);
        let key = Chord::key;

        Self {
            normal: vec![
                // Movement
                (key(K::Char('j')), A::MoveDown),
                (key(K::Down), A::MoveDown),
                (key(K::Char('k')), A::MoveUp),
                (key(K::Up), A::MoveUp),
                (key(K::Char('G')), A::GoToBottom),
                // Navigation. `h`/`l` are back/forward, as documented.
                (key(K::Char('h')), A::Back),
                (key(K::Left), A::Back),
                (key(K::Char('l')), A::Forward),
                (key(K::Right), A::Forward),
                (key(K::Enter), A::Activate),
                (key(K::Tab), A::FocusNext),
                (key(K::BackTab), A::FocusPrevious),
                // Jump straight to a page.
                (key(K::Char('1')), A::Navigate(Route::NowPlaying)),
                (key(K::Char('2')), A::Navigate(Route::Home)),
                (key(K::Char('3')), A::Navigate(Route::Search)),
                (key(K::Char('4')), A::Navigate(Route::Library)),
                (key(K::Char('5')), A::Navigate(Route::Queue)),
                (key(K::Char('/')), A::EditSearch),
                // Play what is highlighted without opening it first.
                (key(K::Char('P')), A::PlaySelection),
                // Playback
                (key(K::Char(' ')), A::TogglePlay),
                (key(K::Char('n')), A::NextTrack),
                (key(K::Char('p')), A::PreviousTrack),
                (key(K::Char('>')), A::SeekForward),
                (key(K::Char('<')), A::SeekBackward),
                (key(K::Char('+')), A::VolumeUp),
                (key(K::Char('=')), A::VolumeUp),
                (key(K::Char('-')), A::VolumeDown),
                (key(K::Char('r')), A::CycleRepeat),
                (key(K::Char('s')), A::ToggleShuffle),
                // Appearance
                (key(K::Char('t')), A::OpenThemes),
                (key(K::Char('v')), A::CycleVisualizer),
                // Overlays and lifecycle
                (key(K::Char('d')), A::OpenDevices),
                (key(K::Char(':')), A::OpenPalette),
                (key(K::Char('?')), A::OpenHelp),
                (key(K::Esc), A::Close),
                (ctrl(K::Char('r')), A::Refresh),
                // `q` steps back, matching `h`; `Q` and ctrl-c leave. Quitting
                // is never one keystroke away from a navigation key.
                (key(K::Char('q')), A::Back),
                (key(K::Char('Q')), A::Quit),
                (ctrl(K::Char('c')), A::Quit),
            ],
            modal: vec![
                (key(K::Char('j')), A::MoveDown),
                (key(K::Down), A::MoveDown),
                (key(K::Char('k')), A::MoveUp),
                (key(K::Up), A::MoveUp),
                (key(K::Char('G')), A::GoToBottom),
                (key(K::Enter), A::Activate),
                (key(K::Esc), A::Close),
                (key(K::Char('q')), A::Close),
                // Pressing the opening key again closes the overlay.
                (key(K::Char('d')), A::Close),
                (key(K::Char('t')), A::Close),
                (key(K::Char('?')), A::Close),
                // `Q` quits from anywhere. Without this an open overlay traps
                // the user, since `q` only closes it.
                (key(K::Char('Q')), A::Quit),
                (ctrl(K::Char('c')), A::Quit),
            ],
            palette: vec![
                (key(K::Esc), A::Close),
                (key(K::Enter), A::PaletteSubmit),
                (key(K::Backspace), A::PaletteBackspace),
                (key(K::Down), A::MoveDown),
                (key(K::Up), A::MoveUp),
                (ctrl(K::Char('n')), A::MoveDown),
                (ctrl(K::Char('p')), A::MoveUp),
                (ctrl(K::Char('c')), A::Quit),
            ],
            // Typing a query. Arrows still move through whatever is already on
            // screen, so results can be walked without leaving the box.
            search: vec![
                (key(K::Esc), A::Close),
                (key(K::Enter), A::SearchSubmit),
                (key(K::Backspace), A::SearchBackspace),
                (key(K::Down), A::MoveDown),
                (key(K::Up), A::MoveUp),
                (key(K::Tab), A::FocusNext),
                (ctrl(K::Char('n')), A::MoveDown),
                (ctrl(K::Char('p')), A::MoveUp),
                (ctrl(K::Char('c')), A::Quit),
            ],
        }
    }
}
