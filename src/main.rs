use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Hello from my ThinkPad

    // env::args() returns an iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}