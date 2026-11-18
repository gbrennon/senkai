use std::cell::RefCell;
use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::SettingsRepository;
use senkai_core::settings::Settings;
use senkai_presentation::cli::settings::{SettingsAction, SettingsCli};

struct FakeSettingsRepository {
    settings: Rc<RefCell<Settings>>,
}

impl FakeSettingsRepository {
    fn new(settings: Settings) -> (Self, Rc<RefCell<Settings>>) {
        let settings = Rc::new(RefCell::new(settings));
        (
            Self {
                settings: Rc::clone(&settings),
            },
            settings,
        )
    }
}

impl SettingsRepository for FakeSettingsRepository {
    fn load(&self) -> Result<Settings> {
        Ok(self.settings.borrow().clone())
    }

    fn save(&self, settings: &Settings) -> Result<()> {
        *self.settings.borrow_mut() = settings.clone();
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn set_action_persists_the_updated_setting() {
        let (repository, state) = FakeSettingsRepository::new(Settings::new());
        let cli = SettingsCli::new(Rc::new(repository));

        cli.run(SettingsAction::Set {
            name: "theme".to_string(),
            value: "dark".to_string(),
        })
        .unwrap();

        assert_eq!(state.borrow().get("theme"), Some("dark"));
    }

    #[test]
    fn get_action_succeeds_for_an_existing_setting() {
        let (repository, _) =
            FakeSettingsRepository::new(Settings::from_values([("theme", "dark")]));
        let cli = SettingsCli::new(Rc::new(repository));

        assert!(
            cli.run(SettingsAction::Get {
                name: "theme".into()
            })
            .is_ok()
        );
    }

    #[test]
    fn get_action_rejects_an_unknown_setting() {
        let (repository, _) = FakeSettingsRepository::new(Settings::new());
        let cli = SettingsCli::new(Rc::new(repository));

        assert!(
            cli.run(SettingsAction::Get {
                name: "missing".into()
            })
            .is_err()
        );
    }
}
