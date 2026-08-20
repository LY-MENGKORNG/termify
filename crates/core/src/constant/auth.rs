use std::time::Duration;

/// How long to wait for the user to finish in the browser.
pub const CONSENT_TIMEOUT: Duration = Duration::from_secs(300);

/// What the browser sees once the code has been captured.
pub const CALLBACK_PAGE: &str = "<!doctype html><meta charset=utf-8><title>termify</title>\
<body style=\"font:16px/1.6 system-ui;display:grid;place-items:center;height:90vh;margin:0\">\
<div><h1 style=\"font-weight:600\">Signed in</h1>\
<p>You can close this tab and return to the terminal.</p></div>";

/// Cap on the request line we will read from the callback connection.
pub const MAX_REQUEST_LINE: u64 = 8 * 1024;
