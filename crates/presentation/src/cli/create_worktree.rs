use std::path::{Path, PathBuf};
use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::{LoadingRunner, MultiplexerPort, RepositoryPort, SelectorRunner};
use senkai_core::worktree_resolver::WorktreeResolver;

use super::repository_locator::RepositoryLocator;
use super::repository_operations::RepositoryOperations;
use super::worktree_runtime::WorktreeRuntime;

pub struct CreateWorktree {
    locator: RepositoryLocator,
    operations: RepositoryOperations,
    runtime: WorktreeRuntime,
    terminal: Rc<dyn MultiplexerPort>,
}

impl CreateWorktree {
    pub fn new(
        terminal: Rc<dyn MultiplexerPort>,
        repository: Rc<dyn RepositoryPort>,
        _selector: Rc<dyn SelectorRunner>,
        loading: Rc<dyn LoadingRunner>,
    ) -> Self {
        Self {
            locator: RepositoryLocator::new(Rc::clone(&terminal), Rc::clone(&repository)),
            operations: RepositoryOperations::new(
                Rc::clone(&repository),
                Rc::clone(&terminal),
                loading,
            ),
            runtime: WorktreeRuntime::new(Rc::clone(&terminal)),
            terminal,
        }
    }

    pub fn run(&self, branch: &str) -> Result<()> {
        let Some(target) = self.prepare(branch)? else {
            return Ok(());
        };
        self.runtime.open(branch, &target)
    }

    pub fn prepare(&self, branch: &str) -> Result<Option<PathBuf>> {
        if branch.trim().is_empty() {
            self.terminal.show_error(
                "Branch name cannot be empty — type a name (e.g. feat/foo) or select an existing worktree",
            )?;
            return Ok(None);
        }
        let (repository_root, target) = self.resolve_target(branch)?;
        self.create_target(&repository_root, &target, branch)
    }

    pub fn resolve_target(&self, branch: &str) -> Result<(PathBuf, PathBuf)> {
        let repository_root = self
            .locator
            .locate()
            .map_err(|error| Error::new(format!("Failed to find repository root: {error}")))?;
        let repository_root_path = Path::new(&repository_root);
        std::env::set_current_dir(repository_root_path)?;
        let worktree_dir = self.terminal.resolve_worktree_dir();
        let project_locator = senkai_core::project_locator::ProjectLocator;
        project_locator.ensure_worktree_directory(repository_root_path, &worktree_dir)?;
        let target = WorktreeResolver.resolve_path(repository_root_path, &worktree_dir, branch);
        Ok((repository_root_path.to_path_buf(), target))
    }

    pub fn create_target(
        &self,
        repository_root: &Path,
        target: &Path,
        branch: &str,
    ) -> Result<Option<PathBuf>> {
        if target.is_dir() {
            return Ok(Some(target.to_path_buf()));
        }
        let default_branch = self.locator.resolve_default_branch(repository_root)?;
        if let Err(error) = self
            .operations
            .create(repository_root, target, branch, &default_branch)
        {
            self.terminal
                .show_error(&format!("Worktree creation failed:\n\n{error:#}"))?;
            return Ok(None);
        }
        Ok(Some(target.to_path_buf()))
    }
}
