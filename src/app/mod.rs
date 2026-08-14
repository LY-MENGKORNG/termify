//! where you get high 🥵

use std::io::{self, Write};

use anyhow::Result;

use crate::{
    api::playback::PlaybackApi,
    app::states::saved::SavedState,
    config::{Conf, paths::Paths},
};

pub mod components;
pub mod pages;
pub mod runner;
pub mod states;

pub struct App {
    saved_state: SavedState,
}

impl App {
    pub async fn new(conf: Conf, paths: &Paths) -> Result<()> {
        let token_path = paths.token_file();

        let _ = io::stdout().flush();

        let saved = SavedState::load(&token_path);

        Ok(())
    }

    pub async fn run<A>(api: A, user: Option<String>, conf: Conf, paths: &Paths) -> Result<()>
    where
        A: PlaybackApi,
    {
        Ok(())
    }
}
