//! The runner utility for the application

use crate::app::states::AppState;

/// Owns the loop's moving parts.
pub struct AppRunner {
    pub state: AppState,
}

impl AppRunner {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        Ok(())
    }
}
