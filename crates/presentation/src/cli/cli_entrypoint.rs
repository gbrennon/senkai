use std::io::IsTerminal;
use std::path::PathBuf;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use clap::Parser;
use senkai_core::error::Result;
use senkai_core::ports::{LoadingRunner, MultiplexerPort, RepositoryPort, SelectorRunner};
use senkai_core::shell_quoter::ShellQuoter;

use super::choose_worktree::ChooseWorktree;
use super::cleanup_worktrees::CleanupWorktrees;
use super::cli_args::CliArgs;
use super::cli_capabilities::CliCapabilities;
use super::cli_command::{CliCommand, TuiCommand};
use super::command::Command;
use super::create_worktree::CreateWorktree;
use super::repository_locator::RepositoryLocator;

pub struct CliEntrypoint {
    terminal: Rc<dyn MultiplexerPort>,
    repository: Rc<dyn RepositoryPort>,
    selector: Rc<dyn SelectorRunner>,
    tui_selector: Rc<dyn SelectorRunner>,
    loading: Rc<dyn LoadingRunner>,
    tui_loading: Rc<dyn LoadingRunner>,
}

impl CliEntrypoint {
    pub fn new(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
        selector: Box<dyn SelectorRunner>,
        loading: Box<dyn LoadingRunner>,
    ) -> Self {
        let selector = Rc::from(selector);
        let loading = Rc::from(loading);
        Self {
            terminal: Rc::from(terminal),
            repository: Rc::from(repository),
            selector: Rc::clone(&selector),
            tui_selector: selector,
            loading: Rc::clone(&loading),
            tui_loading: loading,
        }
    }

    pub fn new_with_capabilities(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
        capabilities: CliCapabilities,
    ) -> Self {
        let (selector, tui_selector, loading, tui_loading) = capabilities.into_parts();
        Self {
            terminal: Rc::from(terminal),
            repository: Rc::from(repository),
            selector: Rc::from(selector),
            tui_selector: Rc::from(tui_selector),
            loading: Rc::from(loading),
            tui_loading: Rc::from(tui_loading),
        }
    }

    pub fn run(&self, args: &[String]) {
        if Self::should_render_help(args) {
            print!("{}", CliArgs::render_help());
            return;
        }
        let (command, rest) = self.parse_args(args);
        let result = if command.is_tui()
            && !std::io::stdin().is_terminal()
            && self.terminal.is_inside_tmux()
        {
            self.spawn_in_popup(args)
                .or_else(|_| self.dispatch(command, &rest))
        } else {
            self.dispatch(command, &rest)
        };
        if let Err(error) = result {
            let _ = self.terminal.show_error(&format!("{error:#}"));
        }
    }

    fn should_render_help(args: &[String]) -> bool {
        args.len() == 1
            || args
                .iter()
                .skip(1)
                .any(|argument| argument == "-h" || argument == "--help")
    }

    pub fn dispatch(&self, command: Command, rest: &[String]) -> Result<()> {
        match command {
            Command::Choose => ChooseWorktree::new(
                Rc::clone(&self.terminal),
                Rc::clone(&self.repository),
                Rc::clone(&self.selector),
                Rc::clone(&self.loading),
            )
            .run(rest.first().map(String::as_str)),
            Command::TuiChoose => ChooseWorktree::new(
                Rc::clone(&self.terminal),
                Rc::clone(&self.repository),
                Rc::clone(&self.tui_selector),
                Rc::clone(&self.tui_loading),
            )
            .run(None),
            Command::CreateWorktree => CreateWorktree::new(
                Rc::clone(&self.terminal),
                Rc::clone(&self.repository),
                Rc::clone(&self.selector),
                Rc::clone(&self.loading),
            )
            .run(rest.first().map(String::as_str).unwrap_or_default()),
            Command::Cleanup => CleanupWorktrees::new(
                Rc::clone(&self.terminal),
                Rc::clone(&self.repository),
                Rc::clone(&self.selector),
                Rc::clone(&self.loading),
            )
            .run(),
            Command::TuiCleanup => CleanupWorktrees::new(
                Rc::clone(&self.terminal),
                Rc::clone(&self.repository),
                Rc::clone(&self.tui_selector),
                Rc::clone(&self.tui_loading),
            )
            .run(),
            Command::TuiDemoLoading => self.tui_loading.run_loading(
                "Demo: loading indicator test…",
                Box::new(|| {
                    thread::sleep(Duration::from_secs(3));
                    Ok(())
                }),
            ),
        }
    }

    pub fn parse_args(&self, args: &[String]) -> (Command, Vec<String>) {
        match CliArgs::try_parse_from(args) {
            Ok(parsed) => {
                let (root, command) = parsed.into_parts();
                if let Some(root) = root {
                    unsafe {
                        std::env::set_var("SENKAI_ROOT", &root);
                        std::env::set_var("TMUX_WORKTREES_ROOT", &root);
                    }
                }
                match command {
                    None => (Command::Choose, Vec::new()),
                    Some(CliCommand::Choose { name }) => {
                        (Command::Choose, name.into_iter().collect())
                    }
                    Some(CliCommand::CreateWorktree { branch }) => {
                        (Command::CreateWorktree, branch.into_iter().collect())
                    }
                    Some(CliCommand::Cleanup) => (Command::Cleanup, Vec::new()),
                    Some(CliCommand::Tui { command }) => match command {
                        None | Some(TuiCommand::Choose) => (Command::TuiChoose, Vec::new()),
                        Some(TuiCommand::Cleanup) => (Command::TuiCleanup, Vec::new()),
                        Some(TuiCommand::DemoLoading) => (Command::TuiDemoLoading, Vec::new()),
                    },
                }
            }
            Err(_) => {
                let command = args
                    .get(1)
                    .map(|value| Command::parse(value))
                    .unwrap_or(Command::Choose);
                let rest = args.iter().skip(2).cloned().collect();
                (command, rest)
            }
        }
    }

    pub fn spawn_in_popup(&self, args: &[String]) -> Result<()> {
        let root = RepositoryLocator::new(Rc::clone(&self.terminal), Rc::clone(&self.repository))
            .locate()?;
        let executable = std::env::current_exe().unwrap_or_else(|_| {
            args.first()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("sk"))
        });
        let command = args
            .iter()
            .skip(1)
            .filter(|argument| !argument.starts_with("--root"))
            .fold(
                ShellQuoter::quote(&executable.to_string_lossy()),
                |command, argument| format!("{command} {}", ShellQuoter::quote(argument)),
            );
        let command = format!("{command} --root={}", ShellQuoter::quote(&root));
        self.terminal.display_popup("60%", "50%", &root, &command)
    }
}
