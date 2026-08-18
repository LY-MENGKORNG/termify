use crate::event::SpotifyEvent;

/// A single iteration's worth of input.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// A key press, resize, mouse event, or focus change.
    Terminal(Box<crossterm::event::Event>),
    /// A reply from the Spotify worker.
    Spotify(SpotifyEvent),
    /// The animation clock advanced.
    Tick,
    /// The process was asked to stop, e.g. by Ctrl-C.
    Shutdown,
}
