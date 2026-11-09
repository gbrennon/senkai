use senkai_core::ports::multiplexer_port::MultiplexerPort;
use senkai_core::ports::repository_port::RepositoryPort;

use crate::application::Application;
use crate::cli::Cli;
use crate::tui::theme::Theme;
use crate::tui::{RatatuiLoading, RatatuiSelector};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
    ) -> Application {
        let theme = Theme::from_multiplexer(terminal.as_ref());
        let cli = Cli::new(
            terminal,
            repository,
            Box::new(RatatuiSelector::new(theme.clone())),
            Box::new(RatatuiLoading::new(theme)),
        );
        Application::new(cli)
    }
}
