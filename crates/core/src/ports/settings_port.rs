use crate::error::Result;
use crate::settings::Settings;

pub trait ListSettings {
    fn execute(&self, settings: &Settings) -> Vec<String>;
}

pub trait GetSettingValue {
    fn execute(&self, settings: &Settings, name: &str) -> Result<Option<String>>;
}

pub trait SetSettingValue {
    fn execute(&self, settings: &Settings, name: &str, value: &str) -> Result<Settings>;
}
