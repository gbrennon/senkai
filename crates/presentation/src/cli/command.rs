#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Choose,
    ChoosePath,
    Cleanup,
    SettingsList,
    SettingsGet,
    SettingsSet,
    TuiChoose,
    TuiCleanup,
    TuiDemoLoading,
    TuiSettings,
}

pub fn has_explicit_tui_argument(args: &[String]) -> bool {
    args.iter().skip(1).any(|argument| argument == "tui")
}

impl Command {
    pub fn is_interactive(self) -> bool {
        matches!(
            self,
            Self::Choose
                | Self::ChoosePath
                | Self::Cleanup
                | Self::TuiChoose
                | Self::TuiCleanup
                | Self::TuiDemoLoading
                | Self::TuiSettings
        )
    }

    pub fn is_tui(self) -> bool {
        matches!(
            self,
            Self::TuiChoose | Self::TuiCleanup | Self::TuiDemoLoading | Self::TuiSettings
        )
    }
}
