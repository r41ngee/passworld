use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,

    #[arg(long, short)]
    pub length: Option<usize>
}

#[derive(Subcommand)]
pub enum Action {
    Generate,
    Init,
}