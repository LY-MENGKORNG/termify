use std::io::{self, Write};

/// Extracts the port from a loopback redirect URI.
pub fn redirect_port(uri: &str) -> Option<u16> {
    // Strip the scheme, then the path, then take whatever follows the last
    let rest = uri.split_once("://").map(|(_, rest)| rest)?;

    let authority = rest.split('/').next()?;

    let port = authority.rsplit_once(':')?.1;

    port.parse().ok()
}

/// Prints the authorization URL, for when the browser cannot be opened.
pub fn announce_url(url: &str) {
    println!("Opening your browser to sign in to Spotify.");
    println!("If it does not open, visit:\n\n  {url}\n");
    let _ = io::stdout().flush();
}
