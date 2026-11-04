use senkai_core::ports::repository_port::RepositoryPort;
use senkai_core::ports::tmux_port::TmuxPort;

use crate::application::Application;
use crate::cli::Cli;
use crate::tui::{RatatuiLoading, RatatuiSelector};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(
        terminal: Box<dyn TmuxPort>,
        repository: Box<dyn RepositoryPort>,
    ) -> Application {
        let cli = Cli::new(
            terminal,
            repository,
            Box::new(RatatuiSelector),
            Box::new(RatatuiLoading),
        );
        Application::new(cli)
    }
}
