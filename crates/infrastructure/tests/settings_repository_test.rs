use std::fs;

use senkai_core::ports::SettingsRepository;
use senkai_core::settings::Settings;
use senkai_infrastructure::settings_repository::TomlSettingsRepository;

mod tests {
    use super::*;

    #[test]
    fn missing_file_loads_empty_settings() {
        let directory = tempfile::tempdir().unwrap();
        let repository = TomlSettingsRepository::new(directory.path().join("settings.toml"));

        assert_eq!(repository.load().unwrap(), Settings::new());
    }

    #[test]
    fn save_and_load_round_trip_string_settings() {
        let directory = tempfile::tempdir().unwrap();
        let repository = TomlSettingsRepository::new(directory.path().join("settings.toml"));
        let settings = Settings::from_values([("editor", "vim"), ("theme", "dark")]);

        repository.save(&settings).unwrap();

        assert_eq!(repository.load().unwrap(), settings);
    }

    #[test]
    fn save_creates_parent_directories() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("nested/settings.toml");
        let repository = TomlSettingsRepository::new(&path);

        repository
            .save(&Settings::from_values([("theme", "dark")]))
            .unwrap();

        assert!(path.exists());
    }

    #[test]
    fn load_rejects_non_string_values() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("settings.toml");
        fs::write(&path, "theme = true\n").unwrap();
        let repository = TomlSettingsRepository::new(path);

        assert!(repository.load().is_err());
    }
}
