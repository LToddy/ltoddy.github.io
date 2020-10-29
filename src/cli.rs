use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(subcommand)]
    Post(Posts),
}

#[derive(Debug, Subcommand)]
pub enum Posts {
    New(NewArgs),
}

#[derive(Debug, Parser)]
pub struct NewArgs {
    pub title: String,
}
