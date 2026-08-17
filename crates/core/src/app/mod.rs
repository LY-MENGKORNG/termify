use crate::{
    config::{Paths, SavedState},
    model::config::Config,
};
use anyhow::Result;
use std::io::{self, Write};

pub async fn init(config: Config, paths: &Paths) -> Result<()> {
    let token_path = paths.token_file();
    let _ = io::stdout().flush();
    let saved = SavedState::load(&paths.state_file());

    prepare().await
}

pub async fn prepare() -> Result<()> {
    Ok(())
}
