use crossterm::event::EventStream;

use crate::{service::Terminal, state::AppState};

pub struct Runner {
    state: AppState,
    terminal: Terminal,
}

impl Runner {
    pub fn new(terminal: Terminal, state: AppState) -> Self {
        Self { terminal, state }
    }

    pub async fn run(mut self) -> std::io::Result<()> {
        let mut terminal_events = EventStream::new();

        loop {
            if self.state.take_dirty() {
                self.draw()?;
            }

            tokio::select! {
                result = tokio::signal::ctrl_c() => {
                    if result.is_err() {
                        tracing::warn!("could not listen for interrupts");
                    }
                    self.state.exit();
                }
            }

            if self.state.is_exiting() {
                break;
            }
        }
        Ok(())
    }

    fn draw(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
