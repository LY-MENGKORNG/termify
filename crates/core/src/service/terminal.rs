//! Terminal lifecycle

use std::io;
use std::panic;
use std::sync::Once;

use ratatui::{DefaultTerminal, Frame};

pub struct Terminal {
    inner: DefaultTerminal,
}

impl Terminal {
    pub fn open() -> io::Result<Self> {
        let inner = ratatui::try_init()?;
        install_panic_logger();
        Ok(Self { inner })
    }

    /// Renders one frame.
    pub fn draw<F>(&mut self, render: F) -> io::Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.inner.draw(render)?;
        Ok(())
    }

    /// Clears the screen, e.g. before handing the terminal back for auth.
    pub fn clear(&mut self) -> io::Result<()> {
        self.inner.clear()
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        ratatui::restore();
    }
}

fn install_panic_logger() {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            tracing::error!(panic = %info, "termify panicked");
            previous(info);
        }));
    });
}
