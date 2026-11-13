use senkai_core::error::{Error, Result};

pub struct RepositoryStatus;

impl RepositoryStatus {
    pub fn verify(status: i32, stdout: String, stderr: String) -> Result<()> {
        if status == 0 {
            Ok(())
        } else {
            Err(Error::new(if stdout.is_empty() { stderr } else { stdout }))
        }
    }
}
