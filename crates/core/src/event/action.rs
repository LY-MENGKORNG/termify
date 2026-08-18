//! Semantic intents.

use crate::state::route::Route;

/// Something the user asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Leave the application.
    Quit,

    /// Move the selection down one row.
    MoveDown,
    /// Move the selection up one row.
    MoveUp,
    /// Jump to the first row.
    GoToTop,
    /// Jump to the last row.
    GoToBottom,
    /// Go back in the navigation history.
    Back,
    /// Go forward in the navigation history.
    Forward,
    /// Open or activate the selection.
    Activate,
    /// Play whatever is highlighted, without opening it.
    PlaySelection,
    /// Jump straight to a page.
    Navigate(Route),
    /// Move focus to the next panel.
    FocusNext,
    /// Move focus to the previous panel.
    FocusPrevious,

    /// Play, or pause if already playing.
    TogglePlay,
    /// Skip to the next item.
    NextTrack,
    /// Skip to the previous item.
    PreviousTrack,
    /// Seek forward by the configured step.
    SeekForward,
    /// Seek backward by the configured step.
    SeekBackward,
    /// Raise the volume by the configured step.
    VolumeUp,
    /// Lower the volume by the configured step.
    VolumeDown,
    /// Cycle repeat off → all → one.
    CycleRepeat,
    /// Toggle shuffle.
    ToggleShuffle,

    /// Open the device picker.
    OpenDevices,
    /// Open the theme picker.
    OpenThemes,
    /// Switch to the next spectrum style.
    CycleVisualizer,
    /// Open the help overlay.
    OpenHelp,
    /// Open the command palette.
    OpenPalette,
    /// Dismiss the topmost overlay, or the oldest notification.
    Close,

    /// Go to the search page and start typing.
    EditSearch,

    /// Type a character into the command palette.
    PaletteInsert(char),
    /// Delete the character before the palette cursor.
    PaletteBackspace,
    /// Run the command currently typed in the palette.
    PaletteSubmit,

    /// Type a character into the search box.
    SearchInsert(char),
    /// Delete the character before the search cursor.
    SearchBackspace,
    /// Run the search currently typed.
    SearchSubmit,

    /// Re-poll playback and devices immediately.
    Refresh,
}

impl Action {
    /// Resolves the identifier used in `config.toml`.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        BINDABLE
            .iter()
            .find(|info| info.name == name)
            .map(|info| info.action)
    }

    /// The table entry for this action, when it is bindable.
    #[must_use]
    pub fn info(self) -> Option<&'static ActionInfo> {
        BINDABLE.iter().find(|info| info.action == self)
    }

    /// Stable identifier, or `"—"` for actions that cannot be bound.
    #[must_use]
    pub fn name(self) -> &'static str {
        self.info().map_or("—", |info| info.name)
    }

    /// Human-readable description, or the identifier when there is none.
    #[must_use]
    pub fn description(self) -> &'static str {
        self.info().map_or("—", |info| info.description)
    }
}

/// A bindable action together with the text the UI shows for it.
#[derive(Debug, Clone, Copy)]
pub struct ActionInfo {
    /// The action itself.
    pub action: Action,
    /// Stable identifier used in `config.toml` and the command palette.
    pub name: &'static str,
    /// Human-readable description for the help overlay.
    pub description: &'static str,
}

/// Every action a user may bind a key to, in the order the help overlay shows.
pub const BINDABLE: &[ActionInfo] = &[
    ActionInfo {
        action: Action::MoveDown,
        name: "move_down",
        description: "Move down",
    },
    ActionInfo {
        action: Action::MoveUp,
        name: "move_up",
        description: "Move up",
    },
    ActionInfo {
        action: Action::GoToTop,
        name: "go_to_top",
        description: "Jump to first",
    },
    ActionInfo {
        action: Action::GoToBottom,
        name: "go_to_bottom",
        description: "Jump to last",
    },
    ActionInfo {
        action: Action::Activate,
        name: "activate",
        description: "Open selection",
    },
    ActionInfo {
        action: Action::PlaySelection,
        name: "play_selection",
        description: "Play selection",
    },
    ActionInfo {
        action: Action::Back,
        name: "back",
        description: "Back",
    },
    ActionInfo {
        action: Action::Forward,
        name: "forward",
        description: "Forward",
    },
    ActionInfo {
        action: Action::FocusNext,
        name: "focus_next",
        description: "Next panel",
    },
    ActionInfo {
        action: Action::FocusPrevious,
        name: "focus_previous",
        description: "Previous panel",
    },
    ActionInfo {
        action: Action::TogglePlay,
        name: "toggle_play",
        description: "Play / pause",
    },
    ActionInfo {
        action: Action::NextTrack,
        name: "next_track",
        description: "Next track",
    },
    ActionInfo {
        action: Action::PreviousTrack,
        name: "previous_track",
        description: "Previous track",
    },
    ActionInfo {
        action: Action::SeekForward,
        name: "seek_forward",
        description: "Seek forward",
    },
    ActionInfo {
        action: Action::SeekBackward,
        name: "seek_backward",
        description: "Seek backward",
    },
    ActionInfo {
        action: Action::VolumeUp,
        name: "volume_up",
        description: "Volume up",
    },
    ActionInfo {
        action: Action::VolumeDown,
        name: "volume_down",
        description: "Volume down",
    },
    ActionInfo {
        action: Action::CycleRepeat,
        name: "cycle_repeat",
        description: "Cycle repeat",
    },
    ActionInfo {
        action: Action::ToggleShuffle,
        name: "toggle_shuffle",
        description: "Toggle shuffle",
    },
    ActionInfo {
        action: Action::OpenDevices,
        name: "open_devices",
        description: "Choose device",
    },
    ActionInfo {
        action: Action::OpenThemes,
        name: "open_themes",
        description: "Choose theme",
    },
    ActionInfo {
        action: Action::CycleVisualizer,
        name: "cycle_visualizer",
        description: "Cycle spectrum style",
    },
    ActionInfo {
        action: Action::OpenPalette,
        name: "open_palette",
        description: "Command palette",
    },
    ActionInfo {
        action: Action::OpenHelp,
        name: "open_help",
        description: "Help",
    },
    ActionInfo {
        action: Action::Refresh,
        name: "refresh",
        description: "Refresh now",
    },
    ActionInfo {
        action: Action::Close,
        name: "close",
        description: "Close overlay",
    },
    ActionInfo {
        action: Action::Quit,
        name: "quit",
        description: "Quit",
    },
    ActionInfo {
        action: Action::Navigate(Route::NowPlaying),
        name: "go_now_playing",
        description: "Go to now playing",
    },
    ActionInfo {
        action: Action::Navigate(Route::Home),
        name: "go_home",
        description: "Go to home",
    },
    ActionInfo {
        action: Action::Navigate(Route::Search),
        name: "go_search",
        description: "Go to search",
    },
    ActionInfo {
        action: Action::EditSearch,
        name: "edit_search",
        description: "Search",
    },
    ActionInfo {
        action: Action::Navigate(Route::Library),
        name: "go_library",
        description: "Go to library",
    },
    ActionInfo {
        action: Action::Navigate(Route::Queue),
        name: "go_queue",
        description: "Go to queue",
    },
];
