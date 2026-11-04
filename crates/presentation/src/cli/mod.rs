use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use tmux_worktrees_core::error::{Error, Result};
use tmux_worktrees_core::{
    merge_checker::MergeChecker,
    ports::{repository_port::RepositoryPort, tmux_port::TmuxPort},
    project_locator::ProjectLocator,
    selector::{SelectionResult, Selector},
    shell_quoter::ShellQuoter,
    worktree_resolver::WorktreeResolver,
};

pub mod args;
pub mod command;

use crate::tui::loading_port::LoadingRunner;
use crate::tui::selector_port::SelectorRunner;
pub use args::{CliArgs, CliCommand};
use clap::Parser;
pub use command::Command;

pub struct Cli {
    terminal: Box<dyn TmuxPort>,
    repository: Box<dyn RepositoryPort>,
    selector: Box<dyn SelectorRunner>,
    loading: Box<dyn LoadingRunner>,
}

impl Cli {
    pub fn new(
        terminal: Box<dyn TmuxPort>,
        repository: Box<dyn RepositoryPort>,
        selector: Box<dyn SelectorRunner>,
        loading: Box<dyn LoadingRunner>,
    ) -> Self {
        Self {
            terminal,
            repository,
            selector,
            loading,
        }
    }

    pub fn run(&self, args: &[String]) {
        let (cmd, rest) = self.parse_args(args);

        let result = if cmd.is_interactive() && !std::io::stdin().is_terminal() {
            self.spawn_in_popup(args)
        } else {
            self.dispatch(cmd, &rest)
        };

        if let Err(e) = result {
            let _ = self.terminal.show_error(&format!("{e:#}"));
        }
    }

    pub fn dispatch(&self, cmd: Command, rest: &[String]) -> Result<()> {
        match cmd {
            Command::Choose => self.run_choose(),
            Command::CreateWorktree => {
                self.run_create(rest.first().map(String::as_str).unwrap_or_default())
            }
            Command::Cleanup => self.run_cleanup(),
            Command::DemoLoading => self.run_demo_loading(),
        }
    }

    pub fn parse_args(&self, args: &[String]) -> (Command, Vec<String>) {
        match CliArgs::try_parse_from(args) {
            Ok(cli_args) => Self::map_cli_args(cli_args),
            Err(e)
                if e.kind() == clap::error::ErrorKind::DisplayHelp
                    || e.kind() == clap::error::ErrorKind::DisplayVersion =>
            {
                let _ = e.print();
                std::process::exit(0);
            }
            Err(_) => Self::parse_legacy_args(args),
        }
    }

    fn map_cli_args(cli_args: CliArgs) -> (Command, Vec<String>) {
        if let Some(root) = &cli_args.root {
            unsafe { std::env::set_var("TMUX_WORKTREES_ROOT", root) };
        }
        match cli_args.command {
            Some(CliCommand::Choose) | None => (Command::Choose, Vec::new()),
            Some(CliCommand::CreateWorktree { branch }) => {
                (Command::CreateWorktree, branch.into_iter().collect())
            }
            Some(CliCommand::Cleanup) => (Command::Cleanup, Vec::new()),
            Some(CliCommand::DemoLoading) => (Command::DemoLoading, Vec::new()),
        }
    }

    fn parse_legacy_args(args: &[String]) -> (Command, Vec<String>) {
        let mut rest = Vec::new();
        let mut i = 1;
        while i < args.len() {
            let a = &args[i];
            if let Some(root) = a.strip_prefix("--root=") {
                unsafe { std::env::set_var("TMUX_WORKTREES_ROOT", root) };
            } else if a == "--root" {
                if let Some(root) = args.get(i + 1) {
                    unsafe { std::env::set_var("TMUX_WORKTREES_ROOT", root) };
                    i += 1;
                }
            } else {
                rest.push(a.clone());
            }
            i += 1;
        }
        let cmd = rest
            .first()
            .map(|s| s.parse::<Command>().unwrap_or(Command::Choose))
            .unwrap_or(Command::Choose);
        if !rest.is_empty() {
            rest.remove(0);
        }
        (cmd, rest)
    }
}

impl Cli {
    pub fn spawn_in_popup(&self, args: &[String]) -> Result<()> {
        let root = match self.find_repo_root() {
            Ok(r) => r,
            Err(e) => {
                self.terminal.show_error(&format!("{e:#}"))?;
                return Ok(());
            }
        };
        let exe = std::env::current_exe().unwrap_or_else(|_| {
            args.first()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("tmux-worktrees"))
        });
        let mut cmd = ShellQuoter::quote(&exe.to_string_lossy());
        for a in args.iter().skip(1) {
            if a.starts_with("--root") {
                continue;
            }
            cmd.push(' ');
            cmd.push_str(&ShellQuoter::quote(a));
        }
        cmd.push_str(" --root=");
        cmd.push_str(&ShellQuoter::quote(&root));
        let _ = self.terminal.display_popup("60%", "50%", &root, &cmd);
        Ok(())
    }

    pub fn run_choose(&self) -> Result<()> {
        let repo_root = self
            .find_repo_root()
            .map_err(|e| Error::new(format!("Failed to find repository root: {}", e)))?;
        let repo_root_path = Path::new(&repo_root);
        std::env::set_current_dir(repo_root_path)?;
        let worktree_dir = self.terminal.resolve_worktree_dir();
        let worktrees = self.list_worktrees(repo_root_path, &worktree_dir)?;
        let mut existing: Vec<_> = worktrees
            .into_iter()
            .filter_map(|path| {
                let name = path.file_name()?.to_string_lossy().into_owned();
                Some((name, path))
            })
            .collect();
        existing.sort_by(|a, b| a.0.cmp(&b.0));
        let existing_names: Vec<String> = existing.iter().map(|(name, _)| name.clone()).collect();
        let header = if existing_names.is_empty() {
            "No worktrees yet — type a name and press Enter to create"
        } else {
            "Type to filter, Enter to select/create"
        };
        let mut selector = Selector::new(existing_names, true);
        let filtered = selector.filter("");
        let picked = self
            .selector
            .run_selector(&mut selector, &filtered, "Worktree> ", header)?;
        match picked {
            SelectionResult::Selected(i) => {
                let (display_name, worktree_path) = &existing[i];
                let branch = self
                    .worktree_branch(worktree_path)
                    .unwrap_or_else(|_| display_name.clone());
                if !branch.is_empty() && branch != "?" {
                    return self.open_worktree_window(&branch, worktree_path);
                }
                self.run_create(display_name)
            }
            SelectionResult::Custom(branch) => self.run_create(&branch),
            SelectionResult::SelectedMany(_) => Ok(()),
            SelectionResult::Cancelled => Ok(()),
        }
    }

    pub fn run_create(&self, branch: &str) -> Result<()> {
        if branch.trim().is_empty() {
            self.terminal.show_error(
                "Branch name cannot be empty — type a name (e.g. feat/foo) or select an existing worktree",
            )?;
            return Ok(());
        }
        let repo_root = self
            .find_repo_root()
            .map_err(|e| Error::new(format!("Failed to find repository root: {}", e)))?;
        let repo_root_path = Path::new(&repo_root);
        std::env::set_current_dir(repo_root_path)?;
        let worktree_dir = self.terminal.resolve_worktree_dir();
        let project_locator = ProjectLocator;
        project_locator.ensure_worktree_directory(repo_root_path, &worktree_dir)?;
        let target = WorktreeResolver.resolve_path(repo_root_path, &worktree_dir, branch);
        if !target.is_dir() {
            let default_branch = self.resolve_default_branch(repo_root_path)?;
            if let Err(e) = self.create_worktree(repo_root_path, &target, branch, &default_branch) {
                self.terminal
                    .show_error(&format!("Worktree creation failed:\n\n{e:#}"))?;
                return Ok(());
            }
        }
        self.open_worktree_window(branch, &target)
    }

    fn open_worktree_window(&self, branch: &str, target: &Path) -> Result<()> {
        let target_abs = WorktreeResolver.resolve_absolute(target);
        let command = self.terminal.resolve_shell_command();
        self.terminal
            .select_or_create_window(branch, &target_abs, &command)?;
        Ok(())
    }

    fn maybe_fetch_default_branch(&self, repo_root: &Path, default_branch: &str) {
        let auto_fetch = self
            .terminal
            .get_option("worktree-auto-fetch")
            .unwrap_or_else(|| "true".to_string());
        if auto_fetch != "false" {
            let repository = &self.repository;
            let loading = &self.loading;
            let fetch_branch = default_branch.to_string();
            let _ = loading.run_loading(
                "Fetching default branch…",
                Box::new(move || {
                    repository
                        .run_in(repo_root, &["fetch", "origin", &fetch_branch, "--no-tags"])
                        .map(|_| ())
                }),
            );
        }
    }

    fn collect_cleanup_items(
        &self,
        repo_root: &Path,
        worktree_dir: &str,
        default_branch: &str,
    ) -> Result<(Vec<String>, Vec<PathBuf>)> {
        let worktrees = self.list_worktrees(repo_root, worktree_dir)?;
        let mut items = Vec::new();
        let mut paths = Vec::new();
        let merge_checker = MergeChecker;
        for ws in &worktrees {
            let branch = self.worktree_branch(ws).unwrap_or_else(|_| "?".to_string());
            let merged = merge_checker
                .is_merged(ws, default_branch, |dir, remote| {
                    self.repository
                        .run_in(dir, &["merge-base", "--is-ancestor", "HEAD", remote])
                        .map(|(s, _, _)| s == 0)
                        .map_err(|e| format!("{e:#}"))
                })
                .unwrap_or(false);
            let item = if merged {
                format!("✓ merged  | {branch}")
            } else {
                format!("✗ active  | {branch}")
            };
            items.push(item);
            paths.push(ws.clone());
        }
        Ok((items, paths))
    }

    fn confirm_cleanup(&self, selected: &[(PathBuf, String)]) -> Result<bool> {
        let branch_names = selected
            .iter()
            .map(|(_, branch)| branch.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let confirmation_header = format!("Remove {} worktree(s): {branch_names}", selected.len());
        let mut confirmation = Selector::new(vec!["Yes".to_string(), "No".to_string()], false);
        let confirmation_filtered = confirmation.filter("");
        let confirmed = self.selector.run_selector(
            &mut confirmation,
            &confirmation_filtered,
            "Confirm removal> ",
            &confirmation_header,
        )?;
        Ok(matches!(confirmed, SelectionResult::Selected(0)))
    }

    fn execute_cleanup_removal(
        &self,
        repo_root: &Path,
        selected: Vec<(PathBuf, String)>,
    ) -> Result<()> {
        for (ws_dir, branch) in selected {
            if let Err(e) = self.remove_worktree(repo_root, &ws_dir, &branch) {
                self.terminal
                    .show_error(&format!("Worktree removal failed:\n\n{e:#}"))?;
                return Ok(());
            }
            if let Err(e) = self.delete_branch(repo_root, &branch) {
                self.terminal
                    .show_error(&format!("Branch deletion failed:\n\n{e:#}"))?;
                return Ok(());
            }
            let _ = self
                .terminal
                .run(&["display-message", &format!("Removed worktree: {branch}")]);
        }
        Ok(())
    }

    pub fn run_cleanup(&self) -> Result<()> {
        let repo_root = self
            .find_repo_root()
            .map_err(|e| Error::new(format!("Failed to find repository root: {}", e)))?;
        let repo_root_path = Path::new(&repo_root);
        std::env::set_current_dir(repo_root_path)?;
        let worktree_dir = self.terminal.resolve_worktree_dir();
        let default_branch = self.resolve_default_branch(repo_root_path)?;
        self.maybe_fetch_default_branch(repo_root_path, &default_branch);

        let (items, paths) =
            self.collect_cleanup_items(repo_root_path, &worktree_dir, &default_branch)?;
        if items.is_empty() {
            self.terminal.show_error(&format!(
                "No worktrees found in {worktree_dir}/ — press Enter to dismiss"
            ))?;
            return Ok(());
        }
        let items_clone = items.clone();
        let mut selector = Selector::new_multi(items);
        let filtered = selector.filter("");
        let picked = self.selector.run_selector(
            &mut selector,
            &filtered,
            "Remove worktree> ",
            "Select worktrees to remove",
        )?;
        let selected_indices = match picked {
            SelectionResult::SelectedMany(indices) => indices,
            _ => return Ok(()),
        };
        if selected_indices.is_empty() {
            self.terminal.show_error("No worktrees were selected")?;
            return Ok(());
        }

        let selected: Vec<_> = selected_indices
            .into_iter()
            .map(|idx| {
                let branch = WorktreeResolver
                    .extract_branch_from_display(&items_clone[idx])
                    .unwrap_or_default();
                (paths[idx].clone(), branch)
            })
            .collect();

        if !self.confirm_cleanup(&selected)? {
            return Ok(());
        }

        self.execute_cleanup_removal(repo_root_path, selected)
    }
}

impl Cli {
    pub fn run_demo_loading(&self) -> Result<()> {
        self.loading.run_loading(
            "Demo: loading indicator test…",
            Box::new(|| {
                thread::sleep(Duration::from_secs(3));
                Ok(())
            }),
        )
    }

    pub fn find_repo_root(&self) -> Result<String> {
        self.find_repo_root_from(&std::env::current_dir()?)
    }

    pub fn find_repo_root_from(&self, start: &Path) -> Result<String> {
        let project_locator = ProjectLocator;
        if let Some(root) = ProjectLocator::from_env() {
            return Ok(root);
        }
        if let Some(root) = project_locator.by_walking(start) {
            return Ok(root);
        }
        if let Ok(Some(v)) = self.terminal.show_environment("MAIN_PROJECT_PATH")
            && !v.is_empty()
        {
            return Ok(v);
        }
        let dir = self.terminal.current_pane_path()?;
        let project_locator = ProjectLocator;
        if let Some(root) = project_locator.by_walking(Path::new(&dir)) {
            return Ok(root);
        }
        Err(Error::new("Not in a git repository"))
    }

    pub fn resolve_default_branch(&self, repo_root: &Path) -> Result<String> {
        let project_locator = ProjectLocator;
        let global = self
            .repository
            .run_in(repo_root, &["config", "--global", "init.defaultBranch"])
            .ok()
            .and_then(|(s, o, _)| if s == 0 { Some(o) } else { None })
            .unwrap_or_default();
        let local = self
            .repository
            .run_in(repo_root, &["config", "init.defaultBranch"])
            .ok()
            .and_then(|(s, o, _)| if s == 0 { Some(o) } else { None })
            .unwrap_or_default();
        let current = self
            .repository
            .run_in(repo_root, &["branch", "--show-current"])
            .ok()
            .and_then(|(s, o, _)| if s == 0 { Some(o) } else { None })
            .unwrap_or_default();
        Ok(project_locator.determine_default_branch(&global, &local, &current))
    }
}

impl Cli {
    pub fn list_worktrees(
        &self,
        repo_root: &Path,
        _work_dir: &str,
    ) -> Result<Vec<std::path::PathBuf>> {
        let (status, out, _) = self
            .repository
            .run_in(repo_root, &["worktree", "list", "--porcelain"])?;
        if status != 0 {
            return Ok(Vec::new());
        }
        let mut res = Vec::new();
        for line in out.lines() {
            if let Some(p) = line.strip_prefix("worktree ") {
                let p = std::path::PathBuf::from(p.trim());
                if p != repo_root {
                    res.push(p);
                }
            }
        }
        Ok(res)
    }

    pub fn create_worktree(
        &self,
        repo_root: &Path,
        target: &Path,
        branch: &str,
        base: &str,
    ) -> Result<()> {
        let repository = &self.repository;
        let loading = &self.loading;
        let _ = loading.run_loading(
            "Fetching origin…",
            Box::new(move || repository.silent_in(repo_root, &["fetch", "origin", "--quiet"])),
        );
        let mut base = base.to_string();
        let remote_base = format!("refs/remotes/origin/{base}");
        let exists = self
            .repository
            .run_in(
                repo_root,
                &["show-ref", "--verify", "--quiet", &remote_base],
            )
            .map(|(s, _, _)| s == 0)
            .unwrap_or(false);
        if exists {
            base = format!("origin/{base}");
        }
        let (status, stdout, stderr) = self.repository.run_in(
            repo_root,
            &[
                "worktree",
                "add",
                target.to_str().unwrap(),
                "-b",
                branch,
                &base,
            ],
        )?;
        if status != 0 {
            return Err(Error::new(if stdout.is_empty() { stderr } else { stdout }));
        }
        Ok(())
    }

    pub fn remove_worktree(&self, repo_root: &Path, ws_dir: &Path, branch: &str) -> Result<()> {
        if !branch.is_empty() {
            self.terminal.kill_window(branch)?;
        }
        let (status, stdout, stderr) = self.repository.run_in(
            repo_root,
            &["worktree", "remove", "--force", ws_dir.to_str().unwrap()],
        )?;
        if status != 0 {
            return Err(Error::new(if stdout.is_empty() { stderr } else { stdout }));
        }
        Ok(())
    }

    pub fn worktree_branch(&self, ws_dir: &Path) -> Result<String> {
        let (status, out, _) = self
            .repository
            .run_in(ws_dir, &["rev-parse", "--abbrev-ref", "HEAD"])?;
        if status != 0 {
            return Ok("?".to_string());
        }
        Ok(out)
    }

    pub fn delete_branch(&self, repo_root: &Path, branch: &str) -> Result<()> {
        let (status, stdout, stderr) = self
            .repository
            .run_in(repo_root, &["branch", "-D", branch])?;
        if status != 0 {
            return Err(Error::new(if stdout.is_empty() { stderr } else { stdout }));
        }
        Ok(())
    }
}
