//! where you get high 🥵

// use std::io::{self, Write};

use anyhow::Result;

use crate::config::{Conf, paths::Paths};

pub mod components;
pub mod pages;
pub mod runner;
pub mod states;

pub struct App {}

impl App {
    pub async fn init(_conf: Conf, _paths: &Paths) -> Result<()> {
        Ok(())
    }
}
