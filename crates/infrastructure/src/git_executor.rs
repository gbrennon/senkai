use senkai_core::error::Result;

use senkai_core::ports::repository_port::RepositoryPort;

use super::command_runner::{CommandRunner, SystemCommandRunner};

pub struct GitExecutor {
    runner: Box<dyn CommandRunner>,
}

impl GitExecutor {
    pub fn new(runner: Box<dyn CommandRunner>) -> Self {
        Self { runner }
    }

    pub fn run_in(&self, cwd: &std::path::Path, args: &[&str]) -> Result<(i32, String, String)> {
        let out = self.runner.run("git", args, Some(cwd)).map_err(|e| {
            senkai_core::error::Error::new(format!("Failed to run git command: {}", e))
        })?;
        Ok((
            out.status.code().unwrap_or(1),
            String::from_utf8_lossy(&out.stdout).trim().to_string(),
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
        ))
    }

    pub fn silent_in(&self, cwd: &std::path::Path, args: &[&str]) -> Result<()> {
        let _ = self.run_in(cwd, args);
        Ok(())
    }

    pub fn default_branch(&self, cwd: &std::path::Path) -> Result<String> {
        let remote_head = self.run_in(
            cwd,
            &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
        )?;
        if remote_head.0 == 0 && !remote_head.1.is_empty() {
            return Ok(remote_head
                .1
                .strip_prefix("origin/")
                .unwrap_or(&remote_head.1)
                .to_string());
        }
        let local_head = self.run_in(cwd, &["symbolic-ref", "--short", "HEAD"])?;
        if local_head.0 == 0 && !local_head.1.is_empty() {
            return Ok(local_head.1);
        }
        Err(senkai_core::error::Error::new(
            "Unable to determine the repository default branch",
        ))
    }
}

impl Default for GitExecutor {
    fn default() -> Self {
        Self::new(Box::new(SystemCommandRunner))
    }
}

impl RepositoryPort for GitExecutor {
    fn default_branch(&self, dir: &std::path::Path) -> Result<String> {
        self.default_branch(dir)
    }

    fn run_in(&self, dir: &std::path::Path, args: &[&str]) -> Result<(i32, String, String)> {
        self.run_in(dir, args)
    }

    fn silent_in(&self, dir: &std::path::Path, args: &[&str]) -> Result<()> {
        self.silent_in(dir, args)
    }
}
