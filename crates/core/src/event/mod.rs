//! The vocabulary of the application loop.

pub mod action;
pub mod app;
pub mod command;
pub mod notification;
pub mod spotify;

pub use action::{Action, ActionInfo};
pub use app::AppEvent;
pub use command::Command;
pub use notification::{Level, Notification};
pub use spotify::SpotifyEvent;
