use crate::error::Result;
use crate::ports::{GetSettingValue, ListSettings, SetSettingValue};
use crate::settings::Settings;

#[derive(Debug, Default)]
pub struct ListSettingsService;

impl ListSettings for ListSettingsService {
    fn execute(&self, settings: &Settings) -> Vec<String> {
        settings.names()
    }
}

#[derive(Debug, Default)]
pub struct GetSettingValueService;

impl GetSettingValue for GetSettingValueService {
    fn execute(&self, settings: &Settings, name: &str) -> Result<Option<String>> {
        Ok(settings.get(name).map(str::to_owned))
    }
}

#[derive(Debug, Default)]
pub struct SetSettingValueService;

impl SetSettingValue for SetSettingValueService {
    fn execute(&self, settings: &Settings, name: &str, value: &str) -> Result<Settings> {
        settings.clone().with_value(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_service_returns_setting_names() {
        let settings = Settings::from_values([("theme", "dark"), ("editor", "vim")]);

        assert_eq!(
            ListSettingsService.execute(&settings),
            vec!["editor", "theme"]
        );
    }

    #[test]
    fn get_service_returns_a_copy_of_the_setting_value() {
        let settings = Settings::from_values([("theme", "dark")]);

        assert_eq!(
            GetSettingValueService.execute(&settings, "theme").unwrap(),
            Some("dark".into())
        );
    }

    #[test]
    fn get_service_returns_none_for_an_unknown_setting() {
        let settings = Settings::new();

        assert_eq!(
            GetSettingValueService
                .execute(&settings, "missing")
                .unwrap(),
            None
        );
    }

    #[test]
    fn set_service_returns_updated_settings_without_mutating_input() {
        let settings = Settings::from_values([("theme", "dark")]);

        let updated = SetSettingValueService
            .execute(&settings, "theme", "light")
            .unwrap();

        assert_eq!(settings.get("theme"), Some("dark"));
        assert_eq!(updated.get("theme"), Some("light"));
    }
}
