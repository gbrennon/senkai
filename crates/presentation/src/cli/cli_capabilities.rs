use senkai_core::ports::{LoadingRunner, SelectorRunner};

pub type LoadingCapability = Box<dyn LoadingRunner>;
pub type SelectorCapability = Box<dyn SelectorRunner>;

pub struct CliCapabilities {
    selector: SelectorCapability,
    tui_selector: SelectorCapability,
    loading: LoadingCapability,
    tui_loading: LoadingCapability,
}

impl CliCapabilities {
    pub fn new(
        selector: SelectorCapability,
        tui_selector: SelectorCapability,
        loading: LoadingCapability,
        tui_loading: LoadingCapability,
    ) -> Self {
        Self {
            selector,
            tui_selector,
            loading,
            tui_loading,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        SelectorCapability,
        SelectorCapability,
        LoadingCapability,
        LoadingCapability,
    ) {
        (
            self.selector,
            self.tui_selector,
            self.loading,
            self.tui_loading,
        )
    }
}
