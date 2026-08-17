use crate::{constant::DEBOUNCE, model::ui::VisualizerStyle};

use super::SavedState;
use std::{path::PathBuf, time::Instant};

/// Watches the running application and writes state when it settles.
#[derive(Debug)]
pub struct Recorder {
    path: PathBuf,
    /// What the application currently looks like.
    desired: SavedState,
    /// What is actually on disk, as far as we know.
    written: SavedState,
    /// When the pending change becomes worth writing.
    due: Option<Instant>,
}

impl Recorder {
    /// Starts watching, told what was just loaded so the first tick is quiet.
    #[must_use]
    pub fn new(path: PathBuf, loaded: SavedState) -> Self {
        Self {
            path,
            desired: loaded.clone(),
            written: loaded,
            due: None,
        }
    }

    /// Notes what the application looks like now.
    pub fn record(
        &mut self,
        volume: Option<u8>,
        theme: &str,
        visualizer_style: VisualizerStyle,
        now: Instant,
    ) {
        let mut changed = false;

        if let Some(volume) = volume.map(|volume| volume.min(100))
            && self.desired.volume != Some(volume)
        {
            self.desired.volume = Some(volume);
            changed = true;
        }

        // Compared before cloning: this runs on every tick, and the string is
        // the same one almost every time.
        if self.desired.theme.as_deref() != Some(theme) {
            self.desired.theme = Some(theme.to_owned());
            changed = true;
        }

        if self.desired.visualizer_style != Some(visualizer_style) {
            self.desired.visualizer_style = Some(visualizer_style);
            changed = true;
        }

        if changed && self.due.is_none() && self.desired != self.written {
            self.due = Some(now + DEBOUNCE);
        }
    }

    /// Writes if the last change has had time to settle.
    pub fn flush_due(&mut self, now: Instant) {
        if self.due.is_some_and(|due| now >= due) {
            self.flush();
        }
    }

    /// Writes immediately, if there is anything to write.
    pub fn flush(&mut self) {
        self.due = None;

        if self.desired == self.written {
            return;
        }

        match self.desired.write(&self.path) {
            Ok(()) => self.written = self.desired.clone(),
            Err(error) => {
                tracing::warn!(%error, path = %self.path.display(), "could not save state");
            }
        }
    }
}
