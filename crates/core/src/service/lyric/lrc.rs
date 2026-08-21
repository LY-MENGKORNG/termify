//! Reading LRC, the format synced lyrics come in.
//!
//! An LRC file is one line per lyric, each prefixed with the time it is sung:
//!
//! ```text
//! [ti:Finesse]
//! [offset:-500]
//! [00:04.89] Ooh, don't we look good together?
//! [00:08.66][01:12.30] There's a reason why they watch all night long
//! ```
//!
//! Everything here is tolerant. A line whose timestamp cannot be read is
//! skipped rather than failing the file: half a song's lyrics are worth more
//! than none, and these files are written by hand and by strangers.

use std::time::Duration;

/// One line of lyrics, and when it is sung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// Offset from the start of the track.
    pub at: Duration,
    /// The words. Empty for the pauses between verses.
    pub text: String,
}

/// Lyrics for one track.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Lyrics {
    lines: Vec<Line>,
    synced: bool,
}

impl Lyrics {
    /// Parses LRC, sorted by time.
    #[must_use]
    pub fn parse(lrc: &str) -> Self {
        let mut lines = Vec::new();
        let mut offset = 0i64;

        for raw in lrc.lines() {
            let (stamps, text) = split_stamps(raw);

            for stamp in &stamps {
                match stamp {
                    Stamp::At(at) => lines.push(Line {
                        at: shift(*at, offset),
                        text: text.to_owned(),
                    }),
                    // `[offset:-500]` means the words come 500 ms *earlier*.
                    Stamp::Offset(milliseconds) => offset = *milliseconds,
                }
            }
        }

        lines.sort_by_key(|line| line.at);

        Self {
            synced: !lines.is_empty(),
            lines,
        }
    }

    /// Wraps unsynced lyrics, which some tracks are all that is available for.
    #[must_use]
    pub fn plain(text: &str) -> Self {
        Self {
            lines: text
                .lines()
                .map(|line| Line {
                    at: Duration::ZERO,
                    text: line.trim_end().to_owned(),
                })
                .collect(),
            synced: false,
        }
    }

    /// Every line, in time order.
    #[must_use]
    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    /// Whether these lyrics carry timings.
    #[must_use]
    pub const fn is_synced(&self) -> bool {
        self.synced
    }

    /// Whether there is nothing to show.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Index of the line being sung at `position`.
    #[must_use]
    pub fn line_at(&self, position: Duration) -> Option<usize> {
        if !self.synced {
            return None;
        }

        // The last line that has already started.
        let next = self.lines.partition_point(|line| line.at <= position);
        next.checked_sub(1)
    }
}

/// A bracketed prefix: either a time or the file's offset.
enum Stamp {
    At(Duration),
    Offset(i64),
}

/// Splits the leading `[...]` groups from the text after them.
fn split_stamps(raw: &str) -> (Vec<Stamp>, &str) {
    let mut stamps = Vec::new();
    let mut rest = raw.trim_start();

    while let Some(body) = rest.strip_prefix('[') {
        let Some((inside, after)) = body.split_once(']') else {
            break;
        };

        if let Some(stamp) = parse_stamp(inside) {
            stamps.push(stamp);
        }
        rest = after;
    }

    (stamps, rest.trim())
}

/// Reads `mm:ss.xx`, or an `offset:` tag. Anything else is metadata.
fn parse_stamp(inside: &str) -> Option<Stamp> {
    if let Some(value) = inside.strip_prefix("offset:") {
        // Some files write `+500`, which `i64::from_str` rejects.
        let value = value.trim();
        let milliseconds = value.strip_prefix('+').unwrap_or(value).parse().ok()?;
        return Some(Stamp::Offset(milliseconds));
    }

    let (minutes, seconds) = inside.split_once(':')?;
    let minutes: u64 = minutes.trim().parse().ok()?;

    // Hundredths or thousandths, both seen in the wild.
    let (whole, fraction) = match seconds.split_once(['.', ':']) {
        Some((whole, fraction)) => (whole, fraction),
        None => (seconds, ""),
    };

    let whole: u64 = whole.trim().parse().ok()?;
    let millis = match fraction.len() {
        0 => 0,
        1 => fraction.parse::<u64>().ok()? * 100,
        2 => fraction.parse::<u64>().ok()? * 10,
        _ => fraction.get(..3)?.parse::<u64>().ok()?,
    };

    Some(Stamp::At(
        Duration::from_secs(minutes * 60 + whole) + Duration::from_millis(millis),
    ))
}

/// Applies an `offset` tag, in milliseconds, without going below zero.
fn shift(at: Duration, offset: i64) -> Duration {
    if offset >= 0 {
        at.saturating_add(Duration::from_millis(offset.unsigned_abs()))
    } else {
        at.saturating_sub(Duration::from_millis(offset.unsigned_abs()))
    }
}
