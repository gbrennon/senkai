use std::path::Path;
use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::{LoadingRunner, MultiplexerPort, RepositoryPort, SelectorRunner};
use senkai_core::selector::SelectionResult;

use super::create_worktree::CreateWorktree;
use super::repository_locator::RepositoryLocator;
use super::repository_operations::RepositoryOperations;
use super::worktree_runtime::WorktreeRuntime;

pub struct ChooseWorktree {
    locator: RepositoryLocator,
    operations: RepositoryOperations,
    runtime: WorktreeRuntime,
    terminal: Rc<dyn MultiplexerPort>,
    repository: Rc<dyn RepositoryPort>,
    selector: Rc<dyn SelectorRunner>,
    loading: Rc<dyn LoadingRunner>,
}

impl ChooseWorktree {
    pub fn new(
        terminal: Rc<dyn MultiplexerPort>,
        repository: Rc<dyn RepositoryPort>,
        selector: Rc<dyn SelectorRunner>,
        loading: Rc<dyn LoadingRunner>,
    ) -> Self {
        Self {
            locator: RepositoryLocator::new(Rc::clone(&terminal), Rc::clone(&repository)),
            operations: RepositoryOperations::new(
                Rc::clone(&repository),
                Rc::clone(&terminal),
                Rc::clone(&loading),
            ),
            runtime: WorktreeRuntime::new(Rc::clone(&terminal)),
            terminal,
            repository,
            selector,
            loading,
        }
    }

    pub fn run(&self, name: Option<&str>) -> Result<()> {
        let repository_root = self
            .locator
            .locate()
            .map_err(|error| Error::new(format!("Failed to find repository root: {error}")))?;
        let repository_root_path = Path::new(&repository_root);
        std::env::set_current_dir(repository_root_path)?;
        if let Some(name) = name {
            return self.create_worktree(name);
        }
        self.select_existing(repository_root_path)
    }

    pub fn select_existing(&self, repository_root: &Path) -> Result<()> {
        let existing = self
            .operations
            .list(repository_root)?
            .into_iter()
            .filter_map(|path| {
                let name = path.file_name()?.to_string_lossy().into_owned();
                Some((name, path))
            })
            .collect::<Vec<_>>();
        let names = existing
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        let header = if names.is_empty() {
            "No worktrees yet — type a name and press Enter to create"
        } else {
            "Type to filter, Enter to select/create"
        };
        let selection = self.selector.select(names, true, "Worktree> ", header)?;
        self.choose(selection, &existing)
    }

    pub fn choose(
        &self,
        selection: SelectionResult,
        existing: &[(String, std::path::PathBuf)],
    ) -> Result<()> {
        match selection {
            SelectionResult::Selected(index) => {
                let (display_name, worktree_path) = &existing[index];
                let branch = self
                    .operations
                    .branch(worktree_path)
                    .unwrap_or_else(|_| display_name.clone());
                match branch.is_empty() || branch == "?" {
                    true => self.create_worktree(display_name),
                    false => self.runtime.open(&branch, worktree_path),
                }
            }
            SelectionResult::Custom(branch) => self.create_worktree(&branch),
            SelectionResult::SelectedMany(_) | SelectionResult::Cancelled => Ok(()),
        }
    }

    pub fn create_worktree(&self, branch: &str) -> Result<()> {
        CreateWorktree::new(
            Rc::clone(&self.terminal),
            Rc::clone(&self.repository),
            Rc::clone(&self.selector),
            Rc::clone(&self.loading),
        )
        .run(branch)
    }
}
