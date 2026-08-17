use ratatui::{DefaultTerminal, Frame};
use std::{io, panic, sync::Once};

/// Owns the terminal for as long as the application runs.
pub struct Terminal {
    inner: DefaultTerminal,
}

impl Terminal {
    /// Enables raw mode, switches to the alternate screen, and installs hooks.
    pub fn open() -> io::Result<Self> {
        let inner = ratatui::try_init()?;
        Self::install_panic_logger();
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

    /// Chains a logging hook in front of whichever hook is currently installed.
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
}

impl Drop for Terminal {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
