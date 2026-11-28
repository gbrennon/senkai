use senkai_core::ports::{LoadingRunner, SelectorRunner};

use crate::tui::SettingsDisplay;

pub type LoadingCapability = Box<dyn LoadingRunner>;
pub type SelectorCapability = Box<dyn SelectorRunner>;
pub type SettingsDisplayCapability = Box<dyn SettingsDisplay>;

pub struct CliCapabilities {
    selector: SelectorCapability,
    tui_selector: SelectorCapability,
    loading: LoadingCapability,
    tui_loading: LoadingCapability,
    settings_display: SettingsDisplayCapability,
}

impl CliCapabilities {
    pub fn new(
        selector: SelectorCapability,
        tui_selector: SelectorCapability,
        loading: LoadingCapability,
        tui_loading: LoadingCapability,
        settings_display: SettingsDisplayCapability,
    ) -> Self {
        Self {
            selector,
            tui_selector,
            loading,
            tui_loading,
            settings_display,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        SelectorCapability,
        SelectorCapability,
        LoadingCapability,
        LoadingCapability,
        SettingsDisplayCapability,
    ) {
        (
            self.selector,
            self.tui_selector,
            self.loading,
            self.tui_loading,
            self.settings_display,
        )
    }
}
