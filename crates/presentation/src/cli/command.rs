#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Choose,
    CreateWorktree,
    Cleanup,
    TuiChoose,
    TuiCleanup,
    TuiDemoLoading,
}

impl Command {
    pub fn parse(value: &str) -> Self {
        match value {
            "create-worktree" | "create" => Self::CreateWorktree,
            "cleanup" => Self::Cleanup,
            "tui" => Self::TuiChoose,
            _ => Self::Choose,
        }
    }

    pub fn is_interactive(self) -> bool {
        matches!(
            self,
            Self::Choose
                | Self::Cleanup
                | Self::TuiChoose
                | Self::TuiCleanup
                | Self::TuiDemoLoading
        )
    }

    pub fn is_tui(self) -> bool {
        matches!(
            self,
            Self::TuiChoose | Self::TuiCleanup | Self::TuiDemoLoading
        )
    }
}
