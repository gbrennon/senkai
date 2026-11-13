use std::path::{Path, PathBuf};
use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::{LoadingRunner, MultiplexerPort, RepositoryPort};

use super::repository_base::RepositoryBase;
use super::repository_cleanup::RepositoryCleanup;
use super::repository_status::RepositoryStatus;

pub struct RepositoryOperations {
    repository: Rc<dyn RepositoryPort>,
    terminal: Rc<dyn MultiplexerPort>,
    loading: Rc<dyn LoadingRunner>,
}

impl RepositoryOperations {
    pub fn new(
        repository: Rc<dyn RepositoryPort>,
        terminal: Rc<dyn MultiplexerPort>,
        loading: Rc<dyn LoadingRunner>,
    ) -> Self {
        Self {
            repository,
            terminal,
            loading,
        }
    }

    pub fn list(&self, repository_root: &Path) -> Result<Vec<PathBuf>> {
        let (status, output, _) = self
            .repository
            .run_in(repository_root, &["worktree", "list", "--porcelain"])?;
        Ok(if status == 0 {
            output
                .lines()
                .filter_map(|line| line.strip_prefix("worktree "))
                .map(PathBuf::from)
                .filter(|path| path != repository_root)
                .collect()
        } else {
            Vec::new()
        })
    }

    pub fn create(
        &self,
        repository_root: &Path,
        target: &Path,
        branch: &str,
        base: &str,
    ) -> Result<()> {
        let repository = Rc::clone(&self.repository);
        let _ = self.loading.run_loading(
            "Fetching origin…",
            Box::new(move || {
                repository.silent_in(repository_root, &["fetch", "origin", "--quiet"])
            }),
        );
        let base = RepositoryBase::new(Rc::clone(&self.repository)).resolve(repository_root, base);
        let (status, stdout, stderr) = self.repository.run_in(
            repository_root,
            &[
                "worktree",
                "add",
                target.to_str().unwrap_or_default(),
                "-b",
                branch,
                &base,
            ],
        )?;
        RepositoryStatus::verify(status, stdout, stderr)
    }

    pub fn branch(&self, worktree: &Path) -> Result<String> {
        let (status, output, _) = self
            .repository
            .run_in(worktree, &["rev-parse", "--abbrev-ref", "HEAD"])?;
        Ok(if status == 0 { output } else { "?".to_string() })
    }

    pub fn remove_and_delete(
        &self,
        repository_root: &Path,
        worktree: &Path,
        branch: &str,
    ) -> Result<()> {
        RepositoryCleanup::new(Rc::clone(&self.repository), Rc::clone(&self.terminal))
            .remove_and_delete(repository_root, worktree, branch)
    }
}
