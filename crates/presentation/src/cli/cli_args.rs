use std::path::PathBuf;

use clap::Parser;

use super::cli_command::CliCommand;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "sk",
    about = "A tmux plugin for creating, switching between, and cleaning up Git worktrees",
    version
)]
pub struct CliArgs {
    #[arg(long, global = true)]
    root: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<CliCommand>,
}

impl CliArgs {
    pub fn into_parts(self) -> (Option<PathBuf>, Option<CliCommand>) {
        (self.root, self.command)
    }
}
