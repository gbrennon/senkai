use senkai_core::error::Result;
use senkai_core::settings::Settings;

pub trait SettingsDisplay {
    fn display(&self, settings: &Settings) -> Result<()>;
}
