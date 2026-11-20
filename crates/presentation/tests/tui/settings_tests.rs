use senkai_core::settings::Settings;
use senkai_presentation::tui::SettingsTui;

mod tests {
    use super::*;

    #[test]
    fn settings_items_render_sorted_names_and_values() {
        let settings = Settings::from_values([("theme", "dark"), ("editor", "nvim")]);

        assert_eq!(
            SettingsTui::items(&settings),
            vec!["editor = nvim".to_string(), "theme = dark".to_string()]
        );
    }
}
