pub use senkai_infrastructure::Container;
pub use senkai_presentation::{Application, CompositionRoot};

use senkai_infrastructure::settings_repository::TomlSettingsRepository;

pub fn build_app() -> Application {
    let settings_path = TomlSettingsRepository::default_path()
        .expect("Senkai settings require a discoverable user configuration directory");
    CompositionRoot::compose_with_settings(
        Container::build_multiplexer(),
        Container::build_repository(),
        Box::new(TomlSettingsRepository::new(settings_path)),
    )
}
