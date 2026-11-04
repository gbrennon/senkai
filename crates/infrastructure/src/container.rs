use senkai_core::ports::multiplexer_port::MultiplexerPort;
use senkai_core::ports::repository_port::RepositoryPort;

use crate::git_executor::GitExecutor;
use crate::tmux_executor::TmuxExecutor;

pub struct Container;

impl Container {
    pub fn build_multiplexer() -> Box<dyn MultiplexerPort> {
        Box::new(TmuxExecutor::default())
    }

    pub fn build_repository() -> Box<dyn RepositoryPort> {
        Box::new(GitExecutor::default())
    }
}
