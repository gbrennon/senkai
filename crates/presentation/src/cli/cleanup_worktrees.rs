use std::path::{Path, PathBuf};
use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::{LoadingRunner, MultiplexerPort, RepositoryPort, SelectorRunner};

use super::cleanup_selection::CleanupSelection;
use super::repository_locator::RepositoryLocator;
use super::repository_operations::RepositoryOperations;

pub struct CleanupWorktrees {
    locator: RepositoryLocator,
    selection: CleanupSelection,
    operations: RepositoryOperations,
}

impl CleanupWorktrees {
    pub fn new(
        terminal: Rc<dyn MultiplexerPort>,
        repository: Rc<dyn RepositoryPort>,
        selector: Rc<dyn SelectorRunner>,
        loading: Rc<dyn LoadingRunner>,
    ) -> Self {
        Self {
            locator: RepositoryLocator::new(Rc::clone(&terminal), Rc::clone(&repository)),
            selection: CleanupSelection::new(
                Rc::clone(&repository),
                selector,
                Rc::clone(&terminal),
                Rc::clone(&loading),
            ),
            operations: RepositoryOperations::new(repository, Rc::clone(&terminal), loading),
        }
    }

    pub fn run(&self) -> Result<()> {
        let repository_root = self
            .locator
            .locate()
            .map_err(|error| Error::new(format!("Failed to find repository root: {error}")))?;
        let repository_root_path = Path::new(&repository_root);
        std::env::set_current_dir(repository_root_path)?;
        self.cleanup(repository_root_path)
    }

    pub fn cleanup(&self, repository_root_path: &Path) -> Result<()> {
        let Some(selected) = self.select_worktrees(repository_root_path)? else {
            return Ok(());
        };
        self.remove_selected(repository_root_path, selected)
    }

    pub fn select_worktrees(
        &self,
        repository_root_path: &Path,
    ) -> Result<Option<Vec<(PathBuf, String)>>> {
        let default_branch = self.locator.resolve_default_branch(repository_root_path)?;
        let (items, paths) = self
            .selection
            .collect(repository_root_path, &default_branch)?;
        let Some(items) = self.selection.ensure_items(items)? else {
            return Ok(None);
        };
        self.selection.select_and_confirm(items, paths)
    }

    pub fn remove_selected(
        &self,
        repository_root_path: &Path,
        selected: Vec<(PathBuf, String)>,
    ) -> Result<()> {
        selected.into_iter().try_for_each(|(worktree, branch)| {
            self.operations
                .remove_and_delete(repository_root_path, &worktree, &branch)
        })
    }
}
