use std::path::Path;
use std::process::Command;
use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::MultiplexerPort;
use senkai_core::worktree_resolver::WorktreeResolver;

pub struct WorktreeRuntime {
    terminal: Rc<dyn MultiplexerPort>,
}

impl WorktreeRuntime {
    pub fn new(terminal: Rc<dyn MultiplexerPort>) -> Self {
        Self { terminal }
    }

    pub fn open(&self, branch: &str, target: &Path, print_path: bool) -> Result<()> {
        let target = WorktreeResolver.resolve_absolute(target);
        if print_path {
            println!("{target}");
            return Ok(());
        }
        if self.terminal.is_inside_tmux() {
            let command = self.terminal.resolve_shell_command();
            return self
                .terminal
                .select_or_create_window(branch, &target, &command);
        }

        let shell = self.terminal.resolve_shell_command();
        let status = Command::new(shell)
            .arg("-i")
            .current_dir(&target)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(Error::new(format!("Shell exited with status {status}")))
        }
    }
}
