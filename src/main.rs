#[macro_use] extern crate log;
extern crate env_logger;
extern crate clap;

mod cli;
mod centroid;
mod filter;

fn main() {
    cli::parse();
//    env_logger::init();

}


