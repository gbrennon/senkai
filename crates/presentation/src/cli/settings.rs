use std::rc::Rc;

use senkai_core::error::{Error, Result};
use senkai_core::ports::{GetSettingValue, ListSettings, SetSettingValue, SettingsRepository};
use senkai_core::settings::Settings;
use senkai_core::settings_application::{
    GetSettingValueService, ListSettingsService, SetSettingValueService,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsAction {
    List,
    Get { name: String },
    Set { name: String, value: String },
}

pub struct SettingsCli {
    repository: Rc<dyn SettingsRepository>,
}

impl SettingsCli {
    pub fn new(repository: Rc<dyn SettingsRepository>) -> Self {
        Self { repository }
    }

    pub fn run(&self, action: SettingsAction) -> Result<()> {
        let settings = self.repository.load()?;
        match action {
            SettingsAction::List => self.list(&settings),
            SettingsAction::Get { name } => self.get(&settings, &name),
            SettingsAction::Set { name, value } => self.set(&settings, &name, &value),
        }
    }

    fn list(&self, settings: &Settings) -> Result<()> {
        for name in ListSettingsService.execute(settings) {
            println!("{name}");
        }
        Ok(())
    }

    fn get(&self, settings: &Settings, name: &str) -> Result<()> {
        let value = GetSettingValueService
            .execute(settings, name)?
            .ok_or_else(|| Error::new(format!("Setting '{name}' was not found")))?;
        println!("{value}");
        Ok(())
    }

    fn set(&self, settings: &Settings, name: &str, value: &str) -> Result<()> {
        let updated = SetSettingValueService.execute(settings, name, value)?;
        self.repository.save(&updated)
    }
}
