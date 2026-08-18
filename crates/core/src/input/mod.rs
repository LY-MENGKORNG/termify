//! Turning key presses into [`crate::events::Action`]s.

pub mod chord;
pub mod keymap;
pub mod mode;

pub use chord::{Chord, ParseChordError, Pending};
pub use keymap::KeyMap;
pub use mode::Mode;
