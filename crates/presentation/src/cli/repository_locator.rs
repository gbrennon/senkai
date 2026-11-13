use std::path::Path;
use std::rc::Rc;

use super::repository_candidates::RepositoryCandidates;
use senkai_core::error::{Error, Result};
use senkai_core::ports::{MultiplexerPort, RepositoryPort};

pub struct RepositoryLocator {
    terminal: Rc<dyn MultiplexerPort>,
    repository: Rc<dyn RepositoryPort>,
}

impl RepositoryLocator {
    pub fn new(terminal: Rc<dyn MultiplexerPort>, repository: Rc<dyn RepositoryPort>) -> Self {
        Self {
            terminal,
            repository,
        }
    }

    pub fn locate(&self) -> Result<String> {
        self.locate_from(&std::env::current_dir()?)
    }

    pub fn locate_from(&self, start: &Path) -> Result<String> {
        RepositoryCandidates::resolve(self.terminal.as_ref(), start)
            .ok_or_else(|| Error::new("Not in a git repository"))
    }

    pub fn resolve_default_branch(&self, repository_root: &Path) -> Result<String> {
        self.repository.default_branch(repository_root)
    }
}
