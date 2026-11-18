use std::rc::Rc;

use senkai_core::error::Result;
use senkai_core::ports::SelectorRunner;
use senkai_core::settings::Settings;

pub struct SettingsTui {
    selector: Rc<dyn SelectorRunner>,
}

impl SettingsTui {
    pub fn new(selector: Rc<dyn SelectorRunner>) -> Self {
        Self { selector }
    }

    pub fn run(&self, settings: &Settings) -> Result<()> {
        let items = Self::items(settings);
        self.selector
            .select(items, false, "Settings> ", "Esc to close")?;
        Ok(())
    }

    pub fn items(settings: &Settings) -> Vec<String> {
        settings
            .names()
            .into_iter()
            .map(|name| {
                let value = settings.get(&name).unwrap_or_default();
                format!("{name} = {value}")
            })
            .collect()
    }
}
