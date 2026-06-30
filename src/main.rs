mod cli;
mod util;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    #[allow(clippy::match_single_binding)]
    match args.action {
        cli::Action::Generate => {
            println!("{}", util::generate_password(args.length))
        },
    }
}
