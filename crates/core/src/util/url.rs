/// Extracts the port from a loopback redirect URI.
pub fn redirect_port(uri: &str) -> Option<u16> {
    // Strip the scheme, then the path, then take what follows the last colon.
    // Hand-rolled to keep `url` an indirect dependency.
    let rest = uri.split_once("://").map(|(_, rest)| rest)?;
    let authority = rest.split('/').next()?;
    let port = authority.rsplit_once(':')?.1;
    port.parse().ok()
}
