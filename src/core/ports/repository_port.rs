use crate::core::error::Result;
use std::path::Path;

/// Port for repository operations — defines the interface the application layer
/// depends on so infrastructure can be swapped for tests.
pub trait RepositoryPort {
    fn run_in(&self, dir: &Path, args: &[&str]) -> Result<(i32, String, String)>;
    fn silent_in(&self, dir: &Path, args: &[&str]) -> Result<()>;
}
