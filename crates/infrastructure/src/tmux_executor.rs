use senkai_core::error::Result;

use senkai_core::ports::multiplexer_port::MultiplexerPort;

use super::command_runner::{CommandRunner, SystemCommandRunner};

/// Thin adapter around `tmux` commands — every call is forwarded through the
/// injected [`CommandRunner`] so tests can supply a fake and never touch a real
/// tmux server.
pub struct TmuxExecutor {
    pub runner: Box<dyn CommandRunner>,
    target_pane: Option<String>,
    inside_tmux: bool,
}

impl Default for TmuxExecutor {
    fn default() -> Self {
        Self {
            runner: Box::new(SystemCommandRunner),
            target_pane: std::env::var("TMUX_PANE").ok(),
            inside_tmux: std::env::var_os("TMUX").is_some(),
        }
    }
}

impl TmuxExecutor {
    pub fn new(runner: Box<dyn CommandRunner>) -> Self {
        Self {
            runner,
            target_pane: None,
            inside_tmux: std::env::var_os("TMUX").is_some(),
        }
    }

    pub fn new_in_tmux(runner: Box<dyn CommandRunner>) -> Self {
        Self {
            runner,
            target_pane: None,
            inside_tmux: true,
        }
    }

    pub fn new_with_target(runner: Box<dyn CommandRunner>, target_pane: &str) -> Self {
        Self {
            runner,
            target_pane: Some(target_pane.to_string()),
            inside_tmux: true,
        }
    }

    pub fn run(&self, args: &[&str]) -> Result<(i32, String, String)> {
        let out = self.runner.run("tmux", args, None).map_err(|e| {
            senkai_core::error::Error::new(format!("Failed to run tmux command: {}", e))
        })?;
        Ok((
            out.status.code().unwrap_or(1),
            String::from_utf8_lossy(&out.stdout).trim().to_string(),
            String::from_utf8_lossy(&out.stderr).trim().to_string(),
        ))
    }

    pub fn run_ok(&self, args: &[&str]) {
        let _ = self.run(args);
    }

    pub fn get_option(&self, name: &str) -> Option<String> {
        match self.run(&["show-option", "-gv", &format!("@{name}")]) {
            Ok((0, out, _)) if !out.is_empty() => Some(out),
            _ => None,
        }
    }

    pub fn resolve_worktree_dir(&self) -> String {
        self.get_option("worktree-dir")
            .unwrap_or_else(|| ".worktrees".to_string())
    }

    pub fn resolve_shell_command(&self) -> String {
        if let Some(c) = self.get_option("worktree-command") {
            return c;
        }
        let uid = self
            .runner
            .run("id", &["-u"], None)
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        if !uid.is_empty()
            && let Ok(out) = self.runner.run("getent", &["passwd", &uid], None)
            && let Ok(s) = String::from_utf8(out.stdout)
            && let Some(shell) = s.lines().next().and_then(|l| l.split(':').nth(6))
            && !shell.is_empty()
        {
            return shell.to_string();
        }
        if let Ok(shell) = std::env::var("SHELL")
            && !shell.is_empty()
        {
            return shell;
        }
        "/bin/bash".to_string()
    }

    pub fn select_or_create_window(&self, branch: &str, cwd: &str, command: &str) -> Result<()> {
        let win = format!("wt-{branch}");
        let exists = self.window_exists(&win)?;
        let target = self.window_target(&win);
        if exists {
            self.run_ok(&["select-window", "-t", &target]);
            self.run_ok(&["display-message", &format!("Resumed worktree: {branch}")]);
        } else {
            let args = self.new_window_args(&win, cwd, command);
            let references: Vec<&str> = args.iter().map(String::as_str).collect();
            self.run_ok(&references);
            self.run_ok(&["select-window", "-t", &target]);
            self.run_ok(&["display-message", &format!("Created worktree: {branch}")]);
        }
        Ok(())
    }

    pub fn kill_window(&self, branch: &str) -> Result<()> {
        let win = format!("wt-{branch}");
        if self.window_exists(&win)? {
            let target = self.window_target(&win);
            self.run_ok(&["kill-window", "-t", &target]);
        }
        Ok(())
    }

    pub fn window_exists(&self, name: &str) -> Result<bool> {
        let args = self.window_list_args();
        match self.run(&args) {
            Ok((0, out, _)) => Ok(out.lines().any(|l| l.trim() == name)),
            _ => Ok(false),
        }
    }

    pub fn show_error(&self, msg: &str) -> Result<()> {
        eprintln!("{msg}");
        if !self.inside_tmux {
            return Ok(());
        }
        let tmp = std::env::temp_dir().join(format!("senkai-err-{}.txt", std::process::id()));
        let _ = std::fs::write(&tmp, format!("{msg}\n"));
        let quoted = senkai_core::shell_quoter::ShellQuoter::quote(tmp.to_string_lossy().as_ref());
        let cmd = format!(
            "cat {quoted}; echo; echo 'Press any key to close...'; bash -c 'read -r -n 1' 2>/dev/null || true; rm -f {quoted}"
        );
        if self.run(&["display-popup", "-E", "-h", "20", &cmd]).is_ok() {
            return Ok(());
        }
        let _ = self.run(&["display-message", "-d", "5000", &format!("senkai: {msg}")]);
        Ok(())
    }

    pub fn display_popup(&self, width: &str, height: &str, dir: &str, cmd: &str) -> Result<()> {
        let popup_style = self.popup_style(
            "sk-popup-style",
            "popup-style",
            "status-style",
            "fg=default,bg=default",
        );
        let border_style = self.popup_style(
            "sk-popup-border-style",
            "popup-border-style",
            "pane-active-border-style",
            "fg=green",
        );
        let args = [
            "display-popup".to_string(),
            "-E".to_string(),
            "-s".to_string(),
            popup_style,
            "-S".to_string(),
            border_style,
            "-w".to_string(),
            width.to_string(),
            "-h".to_string(),
            height.to_string(),
            "-d".to_string(),
            dir.to_string(),
            cmd.to_string(),
        ];
        let references: Vec<&str> = args.iter().map(String::as_str).collect();
        self.run(&references)?;
        Ok(())
    }

    fn popup_style(
        &self,
        override_option: &str,
        option: &str,
        fallback_option: &str,
        fallback: &str,
    ) -> String {
        self.get_option(override_option)
            .or_else(|| self.tmux_style(option))
            .filter(|style| style != "default")
            .or_else(|| self.tmux_style(fallback_option))
            .filter(|style| style != "default")
            .unwrap_or_else(|| fallback.to_string())
    }

    fn window_target(&self, window: &str) -> String {
        self.target_pane
            .as_deref()
            .map(|pane| format!("{pane}:{window}"))
            .unwrap_or_else(|| window.to_string())
    }

    fn new_window_args(&self, window: &str, cwd: &str, command: &str) -> Vec<String> {
        match self.target_pane.as_deref() {
            Some(pane) => vec![
                "new-window".to_string(),
                "-t".to_string(),
                pane.to_string(),
                "-n".to_string(),
                window.to_string(),
                "-c".to_string(),
                cwd.to_string(),
                command.to_string(),
            ],
            None => vec![
                "new-window".to_string(),
                "-n".to_string(),
                window.to_string(),
                "-c".to_string(),
                cwd.to_string(),
                command.to_string(),
            ],
        }
    }

    fn window_list_args(&self) -> Vec<&str> {
        match self.target_pane.as_deref() {
            Some(pane) => vec!["list-windows", "-t", pane, "-F", "#{window_name}"],
            None => vec!["list-windows", "-F", "#{window_name}"],
        }
    }

    fn tmux_style(&self, option: &str) -> Option<String> {
        match self.run(&["show-options", "-gqv", option]) {
            Ok((0, value, _)) if !value.is_empty() => Some(value),
            _ => None,
        }
    }

    pub fn current_pane_path(&self) -> Result<String> {
        if !self.inside_tmux {
            return std::env::current_dir()
                .map(|path| path.to_string_lossy().into_owned())
                .map_err(|error| senkai_core::error::Error::new(error.to_string()));
        }
        match self.run(&["display-message", "-p", "#{pane_current_path}"]) {
            Ok((0, out, _)) => Ok(out.trim().to_string()),
            _ => std::env::current_dir()
                .map(|p| p.to_string_lossy().into_owned())
                .map_err(|e| senkai_core::error::Error::new(e.to_string())),
        }
    }

    pub fn show_environment(&self, name: &str) -> Result<Option<String>> {
        match self.run(&["show-environment", "-g", name]) {
            Ok((0, out, _)) if out.starts_with(&format!("{}=", name)) => Ok(out
                .strip_prefix(&format!("{}=", name))
                .map(|s| s.trim().to_string())),
            _ => Ok(None),
        }
    }
}

impl MultiplexerPort for TmuxExecutor {
    fn is_inside_tmux(&self) -> bool {
        self.inside_tmux
    }

    fn show_error(&self, msg: &str) -> Result<()> {
        self.show_error(msg)
    }

    fn display_popup(&self, width: &str, height: &str, dir: &str, cmd: &str) -> Result<()> {
        self.display_popup(width, height, dir, cmd)
    }

    fn resolve_worktree_dir(&self) -> String {
        self.resolve_worktree_dir()
    }

    fn resolve_shell_command(&self) -> String {
        self.resolve_shell_command()
    }

    fn select_or_create_window(&self, name: &str, path: &str, command: &str) -> Result<()> {
        self.select_or_create_window(name, path, command)
    }

    fn get_option(&self, key: &str) -> Option<String> {
        self.get_option(key)
    }

    fn show_environment(&self, var: &str) -> Result<Option<String>> {
        self.show_environment(var)
    }

    fn current_pane_path(&self) -> Result<String> {
        self.current_pane_path()
    }

    fn run(&self, args: &[&str]) -> Result<(i32, String, String)> {
        self.run(args)
    }

    fn kill_window(&self, name: &str) -> Result<()> {
        self.kill_window(name)
    }
}
