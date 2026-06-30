mod cli;
mod util;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    #[allow(clippy::match_single_binding)]
    match args.action {
        _ => println!("Nothing ever happens"),
    }
}
