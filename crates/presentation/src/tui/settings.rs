use senkai_core::settings::Settings;

pub struct SettingsTui;

impl SettingsTui {
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
