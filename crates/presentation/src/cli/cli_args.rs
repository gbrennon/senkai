use std::path::PathBuf;

use clap::{CommandFactory, Parser};

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
    pub fn render_help() -> String {
        Self::command().render_help().to_string()
    }

    pub fn into_parts(self) -> (Option<PathBuf>, Option<CliCommand>) {
        (self.root, self.command)
    }
}
