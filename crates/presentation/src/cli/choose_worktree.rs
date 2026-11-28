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
        self.run_internal(name, false)
    }

    pub fn run_with_path_output(&self, name: Option<&str>) -> Result<()> {
        self.run_internal(name, true)
    }

    fn run_internal(&self, name: Option<&str>, print_path: bool) -> Result<()> {
        let repository_root = self
            .locator
            .locate()
            .map_err(|error| Error::new(format!("Failed to find repository root: {error}")))?;
        let repository_root_path = Path::new(&repository_root);
        if let Some(name) = name {
            if let Some((branch, worktree_path)) =
                self.find_existing_worktree(repository_root_path, name)?
            {
                return self.runtime.open(&branch, &worktree_path, print_path);
            }
            return self.create_worktree_with_path_output(name, print_path);
        }
        self.select_existing_with_path_output(repository_root_path, print_path)
    }

    fn find_existing_worktree(
        &self,
        repository_root: &Path,
        name: &str,
    ) -> Result<Option<(String, std::path::PathBuf)>> {
        for worktree_path in self.operations.list(repository_root)? {
            let display_name = worktree_path
                .file_name()
                .map(|value| value.to_string_lossy().into_owned());
            let branch = self.operations.branch(&worktree_path).ok();
            if display_name.as_deref() == Some(name)
                || branch.as_deref().map(str::trim) == Some(name)
            {
                return Ok(Some((
                    branch.unwrap_or_else(|| name.to_string()),
                    worktree_path,
                )));
            }
        }
        Ok(None)
    }

    pub fn select_existing(&self, repository_root: &Path) -> Result<()> {
        self.select_existing_with_path_output(repository_root, false)
    }

    fn select_existing_with_path_output(
        &self,
        repository_root: &Path,
        print_path: bool,
    ) -> Result<()> {
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
        self.choose_with_path_output(selection, &existing, print_path)
    }

    pub fn choose(
        &self,
        selection: SelectionResult,
        existing: &[(String, std::path::PathBuf)],
    ) -> Result<()> {
        self.choose_with_path_output(selection, existing, false)
    }

    fn choose_with_path_output(
        &self,
        selection: SelectionResult,
        existing: &[(String, std::path::PathBuf)],
        print_path: bool,
    ) -> Result<()> {
        match selection {
            SelectionResult::Selected(index) => {
                let (display_name, worktree_path) = &existing[index];
                let branch = self
                    .operations
                    .branch(worktree_path)
                    .unwrap_or_else(|_| display_name.clone());
                self.runtime.open(&branch, worktree_path, print_path)
            }
            SelectionResult::Custom(branch) => {
                self.create_worktree_with_path_output(&branch, print_path)
            }
            SelectionResult::SelectedMany(_) | SelectionResult::Cancelled => Ok(()),
        }
    }

    pub fn create_worktree(&self, branch: &str) -> Result<()> {
        self.create_worktree_with_path_output(branch, false)
    }

    fn create_worktree_with_path_output(&self, branch: &str, print_path: bool) -> Result<()> {
        let create = CreateWorktree::new(
            Rc::clone(&self.terminal),
            Rc::clone(&self.repository),
            Rc::clone(&self.selector),
            Rc::clone(&self.loading),
        );
        if print_path {
            create.run_with_path_output(branch)
        } else {
            create.run(branch)
        }
    }
}
