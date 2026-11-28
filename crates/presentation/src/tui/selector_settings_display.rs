use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::SelectorRunner;
use senkai_core::settings::Settings;

use super::settings::SettingsTui;
use super::settings_display::SettingsDisplay;

pub struct SelectorSettingsDisplay {
    selector: Rc<dyn SelectorRunner>,
}

impl SelectorSettingsDisplay {
    pub fn new(selector: Rc<dyn SelectorRunner>) -> Self {
        Self { selector }
    }
}

impl SettingsDisplay for SelectorSettingsDisplay {
    fn display(&self, settings: &Settings) -> Result<()> {
        self.selector.select(
            SettingsTui::items(settings),
            false,
            "Settings> ",
            "Esc to close",
        )?;
        Ok(())
    }
}
