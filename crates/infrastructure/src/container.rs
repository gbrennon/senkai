use tmux_worktrees_core::ports::repository_port::RepositoryPort;
use tmux_worktrees_core::ports::tmux_port::TmuxPort;

use crate::git_executor::GitExecutor;
use crate::tmux_executor::TmuxExecutor;

pub struct Container;

impl Container {
    pub fn build_tmux() -> Box<dyn TmuxPort> {
        Box::new(TmuxExecutor::default())
    }

    pub fn build_repository() -> Box<dyn RepositoryPort> {
        Box::new(GitExecutor::default())
    }
}
