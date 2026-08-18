/// Lyrics could not be looked up.
#[derive(Debug, thiserror::Error)]
pub enum LyricsError {
    /// The lookup could not be made or did not come back.
    #[error("could not reach the lyrics service")]
    Unreachable,

    /// The service answered with something unexpected.
    #[error("the lyrics service returned an unexpected response ({status})")]
    Unexpected {
        /// HTTP status code.
        status: u16,
    },

    /// The reply could not be read.
    #[error("could not read the lyrics service's reply")]
    Malformed,
}
