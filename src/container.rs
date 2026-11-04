use tmux_worktrees_infrastructure::git_executor::GitExecutor;
use tmux_worktrees_infrastructure::tmux_executor::TmuxExecutor;
use tmux_worktrees_presentation::cli::Cli;
use tmux_worktrees_presentation::tui::{RatatuiLoading, RatatuiSelector};

pub struct Container;

impl Container {
    pub fn build_cli() -> Cli {
        Cli::new(
            Box::new(TmuxExecutor::default()),
            Box::new(GitExecutor::default()),
            Box::new(RatatuiSelector),
            Box::new(RatatuiLoading),
        )
    }
}
