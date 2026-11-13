use std::path::Path;

use senkai_core::ports::MultiplexerPort;
use senkai_core::project_locator::ProjectLocator;

pub struct RepositoryCandidates;

impl RepositoryCandidates {
    pub fn resolve(terminal: &dyn MultiplexerPort, start: &Path) -> Option<String> {
        let locator = ProjectLocator;
        ProjectLocator::from_env()
            .or_else(|| locator.by_walking(start))
            .or_else(|| {
                terminal
                    .show_environment("MAIN_PROJECT_PATH")
                    .ok()
                    .flatten()
                    .filter(|value| !value.is_empty())
                    .and_then(|value| locator.by_walking(Path::new(&value)))
            })
            .or_else(|| {
                terminal
                    .current_pane_path()
                    .ok()
                    .and_then(|path| locator.by_walking(Path::new(&path)))
            })
    }
}
