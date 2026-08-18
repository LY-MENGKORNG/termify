//! Pages, and the history between them.

/// A destination in the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Route {
    /// The current track, in detail.
    NowPlaying,
    /// Landing page.
    Home,
    /// Search.
    Search,
    /// Saved playlists, albums, and artists.
    Library,
    /// The play queue.
    Queue,
    /// One opened collection.
    Detail,
}

impl Route {
    /// Every route the sidebar lists, in order.
    pub const ALL: [Self; 5] = [
        Self::NowPlaying,
        Self::Home,
        Self::Search,
        Self::Library,
        Self::Queue,
    ];

    /// Page title, also used as the sidebar label.
    #[must_use]
    pub const fn title(self) -> &'static str {
        match self {
            Self::NowPlaying => "Now Playing",
            Self::Home => "Home",
            Self::Search => "Search",
            Self::Library => "Library",
            Self::Queue => "Queue",
            // Overridden by the page, which titles itself after whatever is
            // open. This is only the fallback.
            Self::Detail => "Details",
        }
    }

    /// Single-column glyph for the collapsed sidebar.
    #[must_use]
    pub const fn glyph(self) -> &'static str {
        match self {
            Self::NowPlaying => "▶",
            Self::Home => "⌂",
            Self::Search => "⌕",
            Self::Library => "♪",
            Self::Queue => "≡",
            Self::Detail => "›",
        }
    }
}

/// Navigation history: one current route, with a back and a forward stack.
#[derive(Debug, Clone)]
pub struct Router {
    current: Route,
    back: Vec<Route>,
    forward: Vec<Route>,
}

impl Router {
    /// Starts a history at `route`.
    #[must_use]
    pub fn new(route: Route) -> Self {
        Self {
            current: route,
            back: Vec::new(),
            forward: Vec::new(),
        }
    }

    /// The route being displayed.
    #[must_use]
    pub const fn current(&self) -> Route {
        self.current
    }

    /// Navigates to `route`, clearing the forward stack.
    pub fn go_to(&mut self, route: Route) -> bool {
        if self.current == route {
            return false;
        }
        self.back.push(self.current);
        self.forward.clear();
        self.current = route;
        true
    }

    /// Steps back, if there is anywhere to go.
    pub fn go_back(&mut self) -> bool {
        match self.back.pop() {
            Some(previous) => {
                self.forward.push(self.current);
                self.current = previous;
                true
            }
            None => false,
        }
    }

    /// Steps forward, if there is anywhere to go.
    pub fn go_forward(&mut self) -> bool {
        match self.forward.pop() {
            Some(next) => {
                self.back.push(self.current);
                self.current = next;
                true
            }
            None => false,
        }
    }

    /// Whether a back step is possible.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        !self.back.is_empty()
    }

    /// Whether a forward step is possible.
    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        !self.forward.is_empty()
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new(Route::NowPlaying)
    }
}
