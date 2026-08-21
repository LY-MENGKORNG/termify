//! Frame composition.

use ratatui::Frame;
use ratatui::widgets::{Paragraph, Widget};

use crate::input::{KeyMap, Pending};
use crate::layout::Chrome;

/// Draws one frame.
pub fn draw(frame: &mut Frame, state: &AppState, keymap: &KeyMap, pending: Pending) {
    let area = frame.area();
    if area.width == 0 || area.height == 0 {
        return;
    }

    let chrome = Chrome::compute(area, state.settings.sidebar_width);

    // Paint the background once so unwritten cells take the theme rather than
    // whatever the terminal had before.
    Paragraph::new("")
        .style(state.theme.base())
        .render(area, frame.buffer_mut());

    pages::render(frame, chrome.content, state, keymap);
    sidebar::render(frame, chrome.sidebar, state, chrome.sidebar_mode);
    status::render(frame, chrome.status, state, keymap, pending);
    player_bar::render(frame, chrome.player, state);

    // Overlays, innermost last: the palette sits above the page, a modal above
    // the palette, and toasts above everything so a failure is never hidden.
    if let Some(palette_state) = state.palette.as_ref() {
        palette::render(frame, chrome.content, state, palette_state);
    }

    match state.modal.as_ref() {
        Some(Modal::Devices(picker)) => {
            modal::render_devices(frame, area, state, picker, keymap);
        }
        Some(Modal::Themes(picker)) => themes::render(frame, area, state, picker, keymap),
        Some(Modal::Help) => help::render(frame, area, state, keymap),
        None => {}
    }

    notifications::render(frame, area, state);
}
