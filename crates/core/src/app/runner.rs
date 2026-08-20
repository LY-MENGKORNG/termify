use anyhow::Result;

use crate::service::Terminal;

pub struct AppRunner {
    terminal: Terminal,
}

impl AppRunner {
    pub fn new(terminal: Terminal) -> Self {
        Self { terminal }
    }

    pub fn run(mut self) -> Result<()> {
        Ok(())
    }
}
