use clap::{App, Arg};
use std::env;

// Env vars
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start() -> String {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("coverage_vector")
                .short("f")
                .long("coverage-vector")
                .value_name("FILE")
                .help("Path to coverage vector")
                .takes_value(true),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let coverage_vector = matches.value_of("coverage_vector").unwrap();
    String::from(coverage_vector)
}
