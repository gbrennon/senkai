use std::fs;
use std::path::{Path, PathBuf};

use senkai_core::error::{Error, Result};
use senkai_core::ports::SettingsRepository;
use senkai_core::settings::Settings;

type TomlTable = toml::map::Map<String, toml::Value>;

pub struct TomlSettingsRepository {
    path: PathBuf,
}

impl TomlSettingsRepository {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_path() -> Result<PathBuf> {
        if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
            return Ok(PathBuf::from(config_home)
                .join("senkai")
                .join("settings.toml"));
        }

        let home = std::env::var_os("HOME")
            .ok_or_else(|| Error::new("Cannot determine the user home directory"))?;
        Ok(PathBuf::from(home).join(".config/senkai/settings.toml"))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn parse_settings(contents: &str) -> Result<Settings> {
        let table = contents
            .parse::<TomlTable>()
            .map_err(|error| Error::new(format!("Failed to parse settings: {error}")))?;
        let values = table.into_iter().map(|(name, value)| match value.as_str() {
            Some(value) => Ok((name, value.to_string())),
            None => Err(Error::new(format!("Setting '{name}' must be a string"))),
        });
        values
            .collect::<Result<Vec<_>>>()
            .map(Settings::from_values)
    }

    fn serialize_settings(settings: &Settings) -> Result<String> {
        let mut table = TomlTable::new();
        for name in settings.names() {
            let value = settings
                .get(&name)
                .ok_or_else(|| Error::new(format!("Setting '{name}' disappeared while saving")))?;
            table.insert(name, toml::Value::String(value.to_string()));
        }
        toml::to_string_pretty(&table)
            .map_err(|error| Error::new(format!("Failed to serialize settings: {error}")))
    }
}

impl SettingsRepository for TomlSettingsRepository {
    fn load(&self) -> Result<Settings> {
        match fs::read_to_string(&self.path) {
            Ok(contents) => Self::parse_settings(&contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Settings::new()),
            Err(error) => Err(error.into()),
        }
    }

    fn save(&self, settings: &Settings) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, Self::serialize_settings(settings)?)?;
        Ok(())
    }
}
