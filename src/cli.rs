use std::io;

use clap::{Command, Parser, Subcommand};
use clap_complete::generate;
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(author, version, name = "daffodils", about = "Creates a static web from markdown files")]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    #[clap(about = "Generate shell completions for your shell to stdout")]
    Completions(CompletionsArgs),

    #[clap(subcommand)]
    Post(Posts),
}

#[derive(Debug, Subcommand)]
pub enum Posts {
    New(NewArgs),
}

#[derive(Debug, Parser)]
pub struct CompletionsArgs {
    #[arg(value_enum, help = "the shell to generate completions for")]
    pub shell: Shell,
}

impl CompletionsArgs {
    pub fn print_completions(&self, cmd: &mut Command, writer: &mut dyn io::Write) {
        generate(self.shell, cmd, cmd.get_name().to_string(), writer);
    }
}

#[derive(Debug, Parser)]
pub struct NewArgs {
    pub title: String,
}
