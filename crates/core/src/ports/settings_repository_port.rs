use crate::error::Result;
use crate::settings::Settings;

pub trait SettingsRepository {
    fn load(&self) -> Result<Settings>;
    fn save(&self, settings: &Settings) -> Result<()>;
}
