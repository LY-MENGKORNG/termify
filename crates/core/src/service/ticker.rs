//! The animation clock.
//!
//! The tick drives progress interpolation and marquee scrolling. It does *not*
//! drive redraws on its own: the reducer only marks the frame dirty when a
//! rendered value actually changed, so an idle app repaints zero times.

use std::time::Duration;

use tokio::time::{Interval, MissedTickBehavior, interval};

/// A fixed-period clock that never tries to catch up after a stall.
pub struct Ticker {
    inner: Interval,
}

impl Ticker {
    /// Creates a ticker firing every `period`.
    ///
    /// A zero period would busy-loop, so it is clamped to 1 ms.
    #[must_use]
    pub fn new(period: Duration) -> Self {
        let period = period.max(Duration::from_millis(1));
        let mut inner = interval(period);
        // If the loop was blocked (a slow API reply, a huge resize), replaying
        // missed ticks would only produce a burst of redundant redraws.
        inner.set_missed_tick_behavior(MissedTickBehavior::Skip);
        Self { inner }
    }

    /// Waits for the next tick.
    pub async fn tick(&mut self) {
        self.inner.tick().await;
    }
}
