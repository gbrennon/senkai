use std::path::Path;
use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::MultiplexerPort;
use senkai_core::worktree_resolver::WorktreeResolver;

pub struct WorktreeRuntime {
    terminal: Rc<dyn MultiplexerPort>,
}

impl WorktreeRuntime {
    pub fn new(terminal: Rc<dyn MultiplexerPort>) -> Self {
        Self { terminal }
    }

    pub fn open(&self, branch: &str, target: &Path) -> Result<()> {
        self.open_with_path_output(branch, target, false)
    }

    pub fn open_with_path_output(
        &self,
        branch: &str,
        target: &Path,
        print_path: bool,
    ) -> Result<()> {
        let target = WorktreeResolver.resolve_absolute(target);
        if self.terminal.is_inside_tmux() {
            let command = self.terminal.resolve_shell_command();
            self.terminal
                .select_or_create_window(branch, &target, &command)
        } else if print_path {
            println!("{target}");
            Ok(())
        } else {
            std::env::set_current_dir(target).map_err(Into::into)
        }
    }
}
