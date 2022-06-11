#[macro_use] extern crate log;
extern crate env_logger;
extern crate clap;

mod cli;
mod centroid;
mod filter;
mod harris;
mod cameras;

fn main() {
    // Parses and executes the target subcommand (e.g filter, centriod)
    cli::parse();

}


