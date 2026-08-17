//! where you start getting high 🥵

// use std::io::{self, Write};

use anyhow::Result;

use crate::config::{Conf, paths::Paths};

pub mod components;
pub mod pages;
pub mod runner;
pub mod states;

pub struct App {}

impl App {
    pub async fn init(_config: Conf, _paths: &Paths) -> Result<()> {
        Self::prepare().await?;
        Ok(())
    }

    pub async fn prepare() -> Result<()> {
        Ok(())
    }
}
