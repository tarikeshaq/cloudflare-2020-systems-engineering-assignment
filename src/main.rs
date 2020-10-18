//! # Systems Assignment 🦀
//!
//! ## Prerequisites:
//! - Clone the repo ⬇️ using `git clone https://github.com/tarikeshaq/cloudflare-2020-systems-engineering-assignment`
//! - You would need Rust installed, best way to do that is the `rustup` route: https://rustup.rs/
//! - That's it!
//!
//! ## How to run:
//! Convieniently this crate is published on crates.io so you could pull it from there, however, if you want to run it locally
//! simply run:
//!
//! ```
//! cargo run -- --url <URL> --profile <NUMBER>
//! ```
//!
//! (Note, you can also add the `--release` flag to run an optimized version of the crate)
//!
//! And it would run `NUMBER` of HTTP get requests against `URL`, and present you with results.
//!
//! ### See the response
//! In addition to the profiling data, if you'd like to see the response of a request, simply omit the `--profile`.
//! I chose not to show the responses when we profile as that would just clutter the stdout.
//!
//! i.e, if you run `cargo run -- --url https://tarikeshaq.tarikesh.workers.dev/links` it will print out the response
//!
//! ## Example outputs:
//! - When running `cargo run --release -- --url https://tarikeshaq.tarikesh.workers.dev/links --profile 10`:
//! ![Profile of running requests against Cloudflare](./static/cloudflare-worker-response.JPG)
//! - When running `cargo run --release -- --url https://aws.random.cat/meow --profile 10`
//! ![Profile of running requests against random cat images](./static/random-cat.JPG)
//!
//! It seems that the cloudflare website is pretty darn fast 🥳
//!
//! ## License
//! This is a project solving an optional assignment from cloudflare and thus is licensed using the same license. Check it out in [`LICENSE`](./LICENSE)

use clap::{App, Arg};
mod http;
mod profile;
use ansi_term::Color::{Blue, Green, Red};
use http::HttpRequest;
use profile::run_profile;

pub fn main() -> anyhow::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(true)
                .takes_value(true)
                .help("The url that the CLI will query"),
        )
        .arg(
            Arg::with_name("profile")
                .short("p")
                .long("profile")
                .takes_value(true)
                .help("Runs a profile on the sent request"),
        )
        .get_matches();
    let url = matches
        .value_of("url")
        .expect("Url is a required argument, and thus must be present");
    let url = url::Url::parse(&url)?;
    let http_request = HttpRequest::new(&url)?;
    if matches.is_present("profile") {
        run_profile(http_request, matches.value_of("profile").unwrap())
    } else {
        let (amount_read, buff, _) = http_request.run()?;
        println!("Read {} bytes", Red.paint(amount_read.to_string()));
        let split = std::str::from_utf8(&buff)?;
        let mut split = split.split_terminator("\r\n\r\n");
        if let Some(headers) = split.next() {
            println!("Headers: \n{}", Blue.paint(headers));
        } else {
            anyhow::bail!("No headers available");
        }
        if let Some(body) = split.next() {
            println!("Body: \n{}", Green.paint(body));
        } else {
            anyhow::bail!("No body available");
        }
        Ok(())
    }
}
