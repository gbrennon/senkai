use std::path::{Path, PathBuf};
use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::merge_checker::MergeChecker;
use senkai_core::ports::{MultiplexerPort, RepositoryPort, SelectorRunner};
use senkai_core::selector::SelectionResult;
use senkai_core::worktree_resolver::WorktreeResolver;

use super::repository_operations::RepositoryOperations;

pub struct CleanupSelection {
    repository: Rc<dyn RepositoryPort>,
    selector: Rc<dyn SelectorRunner>,
    operations: RepositoryOperations,
    terminal: Rc<dyn MultiplexerPort>,
}

impl CleanupSelection {
    pub fn new(
        repository: Rc<dyn RepositoryPort>,
        selector: Rc<dyn SelectorRunner>,
        terminal: Rc<dyn MultiplexerPort>,
        loading: Rc<dyn senkai_core::ports::LoadingRunner>,
    ) -> Self {
        Self {
            repository: Rc::clone(&repository),
            selector,
            operations: RepositoryOperations::new(repository, Rc::clone(&terminal), loading),
            terminal,
        }
    }

    pub fn collect(
        &self,
        repository_root: &Path,
        default_branch: &str,
    ) -> Result<(Vec<String>, Vec<PathBuf>)> {
        let worktrees = self.operations.list(repository_root)?;
        let values = worktrees
            .iter()
            .map(|worktree| {
                let branch = self
                    .operations
                    .branch(worktree)
                    .unwrap_or_else(|_| "?".to_string());
                let merged = MergeChecker
                    .is_merged(worktree, default_branch, |directory, remote| {
                        self.repository
                            .run_in(directory, &["merge-base", "--is-ancestor", "HEAD", remote])
                            .map(|(status, _, _)| status == 0)
                            .map_err(|error| format!("{error:#}"))
                    })
                    .unwrap_or(false);
                let prefix = if merged { "✓ merged" } else { "✗ active" };
                format!("{prefix}  | {branch}")
            })
            .collect();
        Ok((values, worktrees))
    }

    pub fn confirm(&self, selected: &[(PathBuf, String)]) -> Result<bool> {
        let branches = selected
            .iter()
            .map(|(_, branch)| branch.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let header = format!("Remove {} worktree(s): {branches}", selected.len());
        Ok(matches!(
            self.selector.select(
                vec!["Yes".to_string(), "No".to_string()],
                false,
                "Confirm removal> ",
                &header,
            )?,
            SelectionResult::Selected(0)
        ))
    }

    pub fn ensure_items(&self, items: Vec<String>) -> Result<Option<Vec<String>>> {
        if items.is_empty() {
            self.terminal.show_error("No worktrees found")?;
            return Ok(None);
        }
        Ok(Some(items))
    }

    pub fn select_and_confirm(
        &self,
        items: Vec<String>,
        paths: Vec<PathBuf>,
    ) -> Result<Option<Vec<(PathBuf, String)>>> {
        let Some(selected) = self.select(items, paths)? else {
            return Ok(None);
        };
        if !self.confirm(&selected)? {
            return Ok(None);
        }
        Ok(Some(selected))
    }

    pub fn select(
        &self,
        items: Vec<String>,
        paths: Vec<PathBuf>,
    ) -> Result<Option<Vec<(PathBuf, String)>>> {
        let selected = match self.selector.select_many(
            items.clone(),
            "Remove worktree> ",
            "Select worktrees to remove",
        )? {
            SelectionResult::SelectedMany(indices) if !indices.is_empty() => indices
                .into_iter()
                .map(|index| {
                    let branch = WorktreeResolver
                        .extract_branch_from_display(&items[index])
                        .unwrap_or_default();
                    (paths[index].clone(), branch)
                })
                .collect(),
            _ => return Ok(None),
        };
        Ok(Some(selected))
    }
}
