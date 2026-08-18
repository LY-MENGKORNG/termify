//! Which set of bindings is live.

/// The active binding set.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Mode {
    /// Navigating the application.
    #[default]
    Normal,
    /// Typing into the command palette. Printable keys are text, not commands.
    Palette,
    /// Typing a search query. Printable keys are text, not commands.
    Search,
    /// An overlay is open and owns navigation keys.
    Modal,
}
