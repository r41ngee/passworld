mod cli;
mod util;
mod storage;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Cli::parse();

    #[allow(clippy::match_single_binding)]
    match args.action {
        cli::Action::Generate => {
            println!("{}", util::generate_password(args.length))
        },
        cli::Action::Init => {
            let dir_created = storage::ensure_dir_created()?;
            if !dir_created {
                storage::create_if_not_exists()?;
            }
        }
    }

    Ok(())
}
