extern crate uhm;

use clap::Parser;
use uhm::cli::{self, Args};

fn main() {
    let args = Args::parse();
    cli::run(args);
}
