use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug, Clone, PartialEq, Eq)]
#[command(
    name = "sk",
    about = "A tmux plugin for creating, switching between, and cleaning up Git worktrees",
    version
)]
pub struct CliArgs {
    /// Path to the repository root
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<CliCommand>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    /// Choose a worktree to switch to
    #[command(name = "choose")]
    Choose,

    /// Create a new worktree for a branch
    #[command(name = "create-worktree", alias = "create")]
    CreateWorktree {
        /// Branch name for the worktree
        #[arg(value_name = "BRANCH")]
        branch: Option<String>,
    },

    /// Clean up merged or stale worktrees
    #[command(name = "cleanup")]
    Cleanup,

    /// Show the demo loading indicator
    #[command(name = "demo-loading")]
    DemoLoading,
}
