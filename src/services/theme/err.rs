use std::{io, path::PathBuf};

/// A theme file contained something that is not a color.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("`{value}` is not a color; expected #rrggbb, #rgb, \"default\", or \"indexed:N\"")]
pub struct ParseColorErr {
    /// The rejected text.
    pub value: String,
}

impl ParseColorErr {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

/// A theme could not be loaded. Callers fall back to the built-in dark theme.
#[derive(Debug, thiserror::Error)]
pub enum ThemeErr {
    /// No file of that name exists in the themes directory.
    #[error("no theme named `{name}` in {dir}")]
    NotFound {
        /// Requested theme name.
        name: String,
        /// Directory that was searched.
        dir: PathBuf,
    },

    /// The theme file could not be read.
    #[error("could not read {path}: {source}")]
    Io {
        /// Theme file.
        path: PathBuf,
        /// Underlying I/O failure.
        source: io::Error,
    },

    /// The theme file is not valid TOML, or holds an unparseable color.
    #[error("{path} is not a valid theme:\n{source}")]
    Parse {
        /// Theme file.
        path: PathBuf,
        /// Parser diagnostic.
        source: Box<toml::de::Error>,
    },
}
