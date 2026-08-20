use crate::{
    config::{Paths, SavedState},
    model::config::Config,
    service::Terminal,
};
use anyhow::{Context, Result};
use std::io::{self, Write};
pub mod runner;

pub async fn init(config: Config, paths: &Paths) -> Result<()> {
    let token_path = paths.token_file();
    let _ = io::stdout().flush();
    let saved = SavedState::load(&paths.state_file());

    prepare(config).await
}

pub async fn prepare(config: Config) -> Result<()> {
    let terminal = Terminal::open().context("could not take over the terminal")?;

    runner::Runner::new(terminal)
        .run()
        .await
        .context("Unexpectedly error!")
}
