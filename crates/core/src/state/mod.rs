pub mod lifecycle;
pub mod route;
pub mod setting;

use setting::Settings;

use crate::theme::Theme;

pub struct AppState {
    /// Configuration-derived knobs.
    pub settings: Settings,
    /// Theme
    pub theme: Theme,
    /// Whether to keep running.
    pub lifecycle: lifecycle::Lifecycle,
    /// Set when something visible changed. Private: use [`Self::mark_dirty`].
    dirty: bool,
}

impl AppState {
    /// Builds the initial state.
    #[must_use]
    pub fn new(theme: Theme, settings: Settings) -> Self {
        let mut state = Self {
            theme,
            settings,
            lifecycle: Lifecycle::Running,
            dirty: true,
        };
        state.set_themes(Theme::all_built_in().collect());
        state
    }

    /// Replaces the themes that can be switched to.
    fn set_themes(&mut self, themes: Vec<Theme>) {
        self.themes = themes;

        if !self
            .themes
            .iter()
            .any(|theme| theme.name == self.theme.name)
        {
            self.themes.insert(0, self.theme.clone());
        }
    }

    /// Consumes the dirty flag, reporting whether a repaint is needed.
    pub const fn take_dirty(&mut self) -> bool {
        let dirty = self.dirty;
        self.dirty = false;
        dirty
    }

    /// Whether the loop should exit.
    #[must_use]
    pub fn is_exiting(&self) -> bool {
        self.lifecycle == Lifecycle::Exiting
    }

    /// Asks the loop to exit.
    pub const fn exit(&mut self) {
        self.lifecycle = Lifecycle::Exiting;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(Theme::dark(), Settings::default())
    }
}
