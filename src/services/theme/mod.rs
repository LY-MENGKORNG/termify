use ratatui::style::{Modifier, Style};
use serde::{Deserialize, Serialize};

use super::theme::constant::BUILT_IN;

pub mod color;
pub mod constant;
pub mod dark;
pub mod ember;
pub mod err;
pub mod loader;
pub mod midnight;
pub mod neon;
pub mod palette;
pub mod paper;

/// A named palette plus the styles derived from it.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct Theme {
    /// Display name, shown in the command palette.
    pub name: String,
    /// The raw tokens.
    pub palette: palette::Palette,
}

impl Theme {
    /// The built-in dark theme.
    #[must_use]
    pub fn dark() -> Self {
        Self {
            name: dark::NAME.to_owned(),
            palette: dark::PALETTE,
        }
    }

    /// The compiled-in theme of that name, if there is one.
    ///
    /// Case-insensitive, because `config.toml` is written by hand.
    #[must_use]
    pub fn built_in(name: &str) -> Option<Self> {
        BUILT_IN
            .iter()
            .find(|(known, _)| known.eq_ignore_ascii_case(name))
            .map(|(known, palette)| Self {
                name: (*known).to_owned(),
                palette: *palette,
            })
    }

    /// Every compiled-in theme, in [`BUILT_IN`] order.
    pub fn all_built_in() -> impl Iterator<Item = Self> {
        BUILT_IN.iter().map(|(name, palette)| Self {
            name: (*name).to_owned(),
            palette: *palette,
        })
    }

    /// Background style for the whole frame.
    #[must_use]
    pub fn base(&self) -> Style {
        Style::new()
            .bg(self.palette.bg.color())
            .fg(self.palette.text.color())
    }

    /// A raised surface, such as the sidebar or the player bar.
    #[must_use]
    pub fn surface(&self) -> Style {
        Style::new()
            .bg(self.palette.surface.color())
            .fg(self.palette.text.color())
    }

    /// A floating surface: modals and the command palette.
    #[must_use]
    pub fn overlay(&self) -> Style {
        Style::new()
            .bg(self.palette.overlay.color())
            .fg(self.palette.text.color())
    }

    /// Primary body text.
    #[must_use]
    pub fn text(&self) -> Style {
        Style::new().fg(self.palette.text.color())
    }

    /// Secondary text: artist names, metadata.
    #[must_use]
    pub fn subtext(&self) -> Style {
        Style::new().fg(self.palette.subtext.color())
    }

    /// Tertiary text: hints, disabled entries, separators.
    #[must_use]
    pub fn muted(&self) -> Style {
        Style::new().fg(self.palette.muted.color())
    }

    /// The one saturated color. Used sparingly, or it stops meaning anything.
    #[must_use]
    pub fn accent(&self) -> Style {
        Style::new().fg(self.palette.accent.color())
    }

    /// Secondary accent, for contextual links and non-primary emphasis.
    #[must_use]
    pub fn accent_alt(&self) -> Style {
        Style::new().fg(self.palette.accent_alt.color())
    }

    /// A section heading.
    #[must_use]
    pub fn heading(&self) -> Style {
        Style::new()
            .fg(self.palette.text.color())
            .add_modifier(Modifier::BOLD)
    }

    /// A prominent title, e.g. the current track on the now-playing page.
    #[must_use]
    pub fn title(&self) -> Style {
        Style::new()
            .fg(self.palette.text.color())
            .add_modifier(Modifier::BOLD)
    }

    /// Borders, which the design uses only where a boundary is load-bearing.
    #[must_use]
    pub fn border(&self) -> Style {
        Style::new().fg(self.palette.border.color())
    }

    /// The border of the focused panel.
    #[must_use]
    pub fn border_focused(&self) -> Style {
        Style::new().fg(self.palette.accent.color())
    }

    /// The selected row of a list.
    #[must_use]
    pub fn selected(&self) -> Style {
        Style::new()
            .bg(self.palette.selection.color())
            .fg(self.palette.text.color())
            .add_modifier(Modifier::BOLD)
    }

    /// The selected row of a list that does not have focus.
    #[must_use]
    pub fn selected_inactive(&self) -> Style {
        Style::new()
            .bg(self.palette.selection.color())
            .fg(self.palette.subtext.color())
    }

    /// Elapsed portion of a progress bar.
    #[must_use]
    pub fn progress_filled(&self) -> Style {
        Style::new().fg(self.palette.progress_filled.color())
    }

    /// Remaining portion of a progress bar.
    #[must_use]
    pub fn progress_empty(&self) -> Style {
        Style::new().fg(self.palette.progress_empty.color())
    }

    /// Confirmation.
    #[must_use]
    pub fn success(&self) -> Style {
        Style::new().fg(self.palette.success.color())
    }

    /// Something needs attention but the app carries on.
    #[must_use]
    pub fn warning(&self) -> Style {
        Style::new().fg(self.palette.warning.color())
    }

    /// Something failed.
    #[must_use]
    pub fn error(&self) -> Style {
        Style::new().fg(self.palette.error.color())
    }

    /// Neutral information.
    #[must_use]
    pub fn info(&self) -> Style {
        Style::new().fg(self.palette.accent_alt.color())
    }

    /// The spectrum's color at `height`, `0.0` at the bottom and `1.0` at the top.
    #[must_use]
    pub fn spectrum_at(&self, height: f32) -> Style {
        Style::new().fg(color::Hex::blend(&self.palette.spectrum, height))
    }

    /// A keyboard hint chip, e.g. the `d` in "press d for devices".
    #[must_use]
    pub fn key_hint(&self) -> Style {
        Style::new()
            .fg(self.palette.accent_alt.color())
            .add_modifier(Modifier::BOLD)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
