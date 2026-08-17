//! Where the next page of a long list begins.

/// How to ask for the page after the one in hand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum More {
    /// Offset-based: playlists, saved tracks, playlist and album contents.
    Offset(u32),
    /// Cursor-based: followed artists.
    After(String),
}

/// One page of results, with what is needed to continue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paged<T> {
    /// The items on this page.
    pub items: Vec<T>,
    /// How many exist upstream, when Spotify says.
    pub total: Option<u32>,
    /// Where the next page starts. `None` when this is the last one.
    pub more: Option<More>,
}

impl<T> Paged<T> {
    /// A complete result that needs no continuation.
    #[must_use]
    pub fn only(items: Vec<T>) -> Self {
        let total = u32::try_from(items.len()).ok();

        Self {
            items,
            total,
            more: None,
        }
    }

    /// Converts every item, keeping the pagination as it is.
    #[must_use]
    pub fn map<U>(self, convert: impl FnMut(T) -> U) -> Paged<U> {
        Paged {
            items: self.items.into_iter().map(convert).collect(),
            total: self.total,
            more: self.more,
        }
    }
}

impl<T> Default for Paged<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            total: None,
            more: None,
        }
    }
}
