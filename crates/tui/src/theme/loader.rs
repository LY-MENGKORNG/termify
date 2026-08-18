//! Loading themes from `<config>/themes/*.toml`.

use std::fs;
use std::path::Path;

use termify_core::error::theme::ThemeError;

use super::Theme;

/// Loads a theme by name: a file in `themes/`, else a built-in of that name.
pub fn load(name: &str, themes_dir: &Path) -> Result<Theme, ThemeError> {
    let path = themes_dir.join(format!("{name}.toml"));

    if !path.exists() {
        return Theme::built_in(name).ok_or_else(|| ThemeError::NotFound {
            name: name.to_owned(),
            dir: themes_dir.to_path_buf(),
        });
    }

    let raw = fs::read_to_string(&path).map_err(|source| ThemeError::Io {
        path: path.clone(),
        source,
    })?;

    let mut theme: Theme = toml::from_str(&raw).map_err(|source| ThemeError::Parse {
        path: path.clone(),
        source: Box::new(source),
    })?;

    // A file that omits `name` inherits the default theme's, so anything matching
    // a built-in is really unnamed and the file name identifies it.
    if theme.name.is_empty() || Theme::built_in(&theme.name).is_some() {
        name.clone_into(&mut theme.name);
    }

    Ok(theme)
}

/// Loads every selectable theme: the built-ins, then each `*.toml` found.
#[must_use]
pub fn load_all(themes_dir: &Path) -> (Vec<Theme>, Vec<ThemeError>) {
    let mut themes: Vec<Theme> = Theme::all_built_in().collect();
    let mut errors = Vec::new();

    for stem in stems(themes_dir) {
        match load(&stem, themes_dir) {
            Ok(theme) => match themes.iter_mut().find(|known| known.name == theme.name) {
                Some(known) => *known = theme,
                None => themes.push(theme),
            },
            Err(error) => errors.push(error),
        }
    }

    (themes, errors)
}

/// Lists selectable theme names: the built-ins plus every `*.toml` found.
#[must_use]
pub fn available(themes_dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = super::BUILT_IN
        .iter()
        .map(|(name, _)| (*name).to_owned())
        .collect();

    names.extend(stems(themes_dir));
    names.sort_unstable();
    names.dedup();
    names
}

/// File stems of the `*.toml` files in `themes_dir`, in a stable order.
fn stems(themes_dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(themes_dir) else {
        return Vec::new();
    };

    let mut stems: Vec<String> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(str::to_owned)
        })
        .collect();

    // Directory order is whatever the filesystem feels like; the picker's is not.
    stems.sort_unstable();
    stems.dedup();
    stems
}
