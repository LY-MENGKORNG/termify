pub mod runner;

use crate::{
    config::{Paths, SavedState},
    model::config::Config,
    service::Terminal,
};
use anyhow::{Context, Result};
use std::io::{self, Write};

pub async fn init(config: Config, paths: &Paths) -> Result<()> {
    let _token_path = paths.token_file();
    let _ = io::stdout().flush();
    let saved = SavedState::load(&paths.state_file());

    prepare(config, saved).await
}

pub async fn prepare(_config: Config, _saved: SavedState) -> Result<()> {
    let terminal = Terminal::open().context("could not take over the terminal")?;
    runner::AppRunner::new(terminal).run()
}
