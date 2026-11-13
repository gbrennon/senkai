use std::path::Path;
use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::{MultiplexerPort, RepositoryPort};

use super::repository_status::RepositoryStatus;

pub struct RepositoryCleanup {
    repository: Rc<dyn RepositoryPort>,
    terminal: Rc<dyn MultiplexerPort>,
}

impl RepositoryCleanup {
    pub fn new(repository: Rc<dyn RepositoryPort>, terminal: Rc<dyn MultiplexerPort>) -> Self {
        Self {
            repository,
            terminal,
        }
    }

    pub fn remove_and_delete(
        &self,
        repository_root: &Path,
        worktree: &Path,
        branch: &str,
    ) -> Result<()> {
        self.terminate_window(branch)?;
        self.remove_worktree(repository_root, worktree)?;
        self.delete_branch(repository_root, branch)
    }

    pub fn terminate_window(&self, branch: &str) -> Result<()> {
        (!branch.is_empty())
            .then(|| self.terminal.kill_window(branch))
            .transpose()?;
        Ok(())
    }

    pub fn remove_worktree(&self, repository_root: &Path, worktree: &Path) -> Result<()> {
        let (status, stdout, stderr) = self.repository.run_in(
            repository_root,
            &[
                "worktree",
                "remove",
                "--force",
                worktree.to_str().unwrap_or_default(),
            ],
        )?;
        RepositoryStatus::verify(status, stdout, stderr)
    }

    pub fn delete_branch(&self, repository_root: &Path, branch: &str) -> Result<()> {
        let (status, stdout, stderr) = self
            .repository
            .run_in(repository_root, &["branch", "-D", branch])?;
        RepositoryStatus::verify(status, stdout, stderr)
    }
}
