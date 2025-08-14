//! Simple HTTPS GET
//!
//! This example is a Rust adaptation of the [C example of the same
//! name](https://curl.se/libcurl/c/https.html).

extern crate curl;

use curl::easy::{Easy, HttpVersion};
use std::io::{stdout, Write};

fn main() -> Result<(), curl::Error> {
    let mut curl = Easy::new();

    curl.url("https://cloudflare-quic.com/cdn-cgi/trace")?;
    curl.cainfo("/etc/ssl/certs/ca-certificates.crt")?;
    curl.http_version(HttpVersion::V3)?;
    curl.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })?;

    curl.perform()?;

    Ok(())
}
