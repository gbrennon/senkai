use std::rc::Rc;

use senkai_core::ports::SelectorRunner;
use senkai_core::selector::SelectionResult;
use senkai_core::settings::Settings;
use senkai_presentation::tui::{SelectorSettingsDisplay, SettingsDisplay, SettingsTui};

use crate::common::fakes::selector::FakeSelector;

mod tests {
    use super::*;

    #[test]
    fn settings_items_render_sorted_names_and_values() {
        let settings = Settings::from_values([("theme", "dark"), ("editor", "vim")]);

        assert_eq!(
            SettingsTui::items(&settings),
            vec!["editor = vim".to_string(), "theme = dark".to_string()]
        );
    }

    #[test]
    fn settings_display_delegates_sorted_rows_to_selector() {
        let selector_impl = Rc::new(FakeSelector::new(vec![SelectionResult::Cancelled]));
        let selector: Rc<dyn SelectorRunner> = selector_impl.clone();
        let display: Rc<dyn SettingsDisplay> = Rc::new(SelectorSettingsDisplay::new(selector));
        let settings = Settings::from_values([("theme", "dark"), ("editor", "vim")]);

        display.display(&settings).unwrap();

        assert_eq!(
            selector_impl.calls.borrow().as_slice(),
            &[vec!["editor = vim".to_string(), "theme = dark".to_string()]]
        );
    }
}
