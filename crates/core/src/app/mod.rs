use crate::{
    config::{Paths, SavedState},
    model::config::Config,
    service::Terminal,
    state::{AppState, setting::Settings},
    theme,
};
use anyhow::{Context, Result};
use std::io::{self, Write};
pub mod runner;

pub async fn init(config: Config, paths: &Paths) -> Result<()> {
    let token_path = paths.token_file();
    let _ = io::stdout().flush();
    let saved = SavedState::load(&paths.state_file());

    prepare(config, saved, &paths).await
}

pub async fn prepare(config: Config, saved: SavedState, paths: &Paths) -> Result<()> {
    let theme = match theme::loader::load(saved.theme_or(&config.ui.theme), &paths.themes_dir()) {
        Ok(theme) => theme,
        Err(error) => {
            tracing::warn!(%error, "falling back to the built-in dark theme");
            Theme::dark()
        }
    };
    let settings = Settings::from_config(ui, playback);
    let mut state = AppState::new(theme, settings);

    let terminal = Terminal::open().context("could not take over the terminal")?;

    runner::Runner::new(terminal, state)
        .run()
        .await
        .context("Unexpectedly error!")
}
