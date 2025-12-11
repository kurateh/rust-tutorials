use std::process;

use clap::Parser;
use minigrep::Args;

fn main() {
    let args = Args::parse();

    if let Err(err) = minigrep::run(args) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
