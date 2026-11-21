use senkai_core::ports::SettingsRepository;
use senkai_core::ports::multiplexer_port::MultiplexerPort;
use senkai_core::ports::repository_port::RepositoryPort;

use crate::application::Application;
use crate::capabilities::{PlainLoading, PlainSelector};
use crate::cli::Cli;
use crate::cli::cli_capabilities::CliCapabilities;
use crate::tui::theme::Theme;
use crate::tui::{RatatuiLoading, RatatuiSelector};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
    ) -> Application {
        Self::compose_internal(terminal, repository, None)
    }

    pub fn compose_with_settings(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
        settings_repository: Box<dyn SettingsRepository>,
    ) -> Application {
        Self::compose_internal(terminal, repository, Some(settings_repository))
    }

    fn compose_internal(
        terminal: Box<dyn MultiplexerPort>,
        repository: Box<dyn RepositoryPort>,
        settings_repository: Option<Box<dyn SettingsRepository>>,
    ) -> Application {
        let theme = Theme::from_multiplexer(terminal.as_ref());
        let capabilities = CliCapabilities::new(
            Box::new(PlainSelector::new()),
            Box::new(RatatuiSelector::new(theme.clone())),
            Box::new(PlainLoading::new()),
            Box::new(RatatuiLoading::new(theme)),
        );
        let cli = match settings_repository {
            Some(settings_repository) => Cli::new_with_capabilities_and_settings(
                terminal,
                repository,
                capabilities,
                settings_repository,
            ),
            None => Cli::new_with_capabilities(terminal, repository, capabilities),
        };
        Application::new(cli)
    }
}
