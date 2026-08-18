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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Lyrics;

    /// Shorthand for a position in seconds.
    fn at(seconds: f64) -> Duration {
        Duration::from_millis((seconds * 1000.0) as u64)
    }

    #[test]
    fn reads_timestamps_and_words() {
        let lyrics = Lyrics::parse(
            "[00:04.89] Ooh, don't we look good together?\n\
             [00:08.66] There's a reason why they watch\n",
        );

        assert!(lyrics.is_synced());
        assert_eq!(lyrics.lines().len(), 2);
        assert_eq!(lyrics.lines().first().map(|line| line.at), Some(at(4.89)));
        assert_eq!(
            lyrics.lines().first().map(|line| line.text.as_str()),
            Some("Ooh, don't we look good together?")
        );
    }

    #[test]
    fn metadata_tags_are_ignored_rather_than_read_as_times() {
        let lyrics = Lyrics::parse("[ti:Finesse]\n[ar:Bruno Mars]\n[00:01.00] First\n");

        assert_eq!(lyrics.lines().len(), 1);
        assert_eq!(
            lyrics.lines().first().map(|line| line.text.as_str()),
            Some("First")
        );
    }

    #[test]
    fn a_line_repeated_at_several_times_appears_at_each_of_them() {
        let lyrics = Lyrics::parse("[00:10.00][01:20.00] Chorus\n");

        assert_eq!(lyrics.lines().len(), 2);
        assert_eq!(lyrics.lines().last().map(|line| line.at), Some(at(80.0)));
    }

    #[test]
    fn an_offset_tag_moves_every_line() {
        let early = Lyrics::parse("[offset:-500]\n[00:10.00] Words\n");
        let late = Lyrics::parse("[offset:+500]\n[00:10.00] Words\n");

        assert_eq!(early.lines().first().map(|line| line.at), Some(at(9.5)));
        assert_eq!(late.lines().first().map(|line| line.at), Some(at(10.5)));
    }

    #[test]
    fn lines_are_sorted_even_when_the_file_is_not() {
        let lyrics = Lyrics::parse("[00:20.00] Second\n[00:10.00] First\n");

        let texts: Vec<&str> = lyrics
            .lines()
            .iter()
            .map(|line| line.text.as_str())
            .collect();
        assert_eq!(texts, vec!["First", "Second"]);
    }

    #[test]
    fn the_current_line_follows_the_position() {
        let lyrics = Lyrics::parse("[00:10.00] First\n[00:20.00] Second\n[00:30.00] Third\n");

        // Before the first word, nothing is being sung.
        assert_eq!(lyrics.line_at(at(5.0)), None);
        assert_eq!(lyrics.line_at(at(10.0)), Some(0));
        assert_eq!(lyrics.line_at(at(19.9)), Some(0));
        assert_eq!(lyrics.line_at(at(20.0)), Some(1));
        // And it stays on the last line to the end of the track.
        assert_eq!(lyrics.line_at(at(600.0)), Some(2));
    }

    #[test]
    fn unreadable_timestamps_are_skipped_not_fatal() {
        let lyrics = Lyrics::parse("[oops] Bad\n[00:05.00] Good\n[??:??] Worse\n");

        assert_eq!(lyrics.lines().len(), 1);
        assert_eq!(
            lyrics.lines().first().map(|line| line.text.as_str()),
            Some("Good")
        );
    }

    #[test]
    fn a_file_with_no_timestamps_is_not_synced() {
        let lyrics = Lyrics::parse("Just some words\nand some more\n");

        assert!(!lyrics.is_synced());
        assert!(lyrics.is_empty());
    }

    #[test]
    fn plain_lyrics_can_be_read_but_not_followed() {
        let lyrics = Lyrics::plain("First line\nSecond line\n");

        assert!(!lyrics.is_synced());
        assert_eq!(lyrics.lines().len(), 2);
        // Nothing is "current" when nothing is timed.
        assert_eq!(lyrics.line_at(at(30.0)), None);
    }

    #[test]
    fn minutes_beyond_an_hour_still_read() {
        let lyrics = Lyrics::parse("[75:30.00] Long song\n");

        assert_eq!(
            lyrics.lines().first().map(|line| line.at),
            Some(Duration::from_secs(75 * 60 + 30))
        );
    }

    #[test]
    fn thousandths_are_accepted_as_well_as_hundredths() {
        let lyrics = Lyrics::parse("[00:01.500] Words\n");

        assert_eq!(lyrics.lines().first().map(|line| line.at), Some(at(1.5)));
    }

    #[test]
    fn empty_input_yields_nothing() {
        assert!(Lyrics::parse("").is_empty());
    }
}
