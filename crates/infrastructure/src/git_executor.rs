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
}

impl Default for GitExecutor {
    fn default() -> Self {
        Self::new(Box::new(SystemCommandRunner))
    }
}

impl RepositoryPort for GitExecutor {
    fn run_in(&self, dir: &std::path::Path, args: &[&str]) -> Result<(i32, String, String)> {
        self.run_in(dir, args)
    }

    fn silent_in(&self, dir: &std::path::Path, args: &[&str]) -> Result<()> {
        self.silent_in(dir, args)
    }
}
