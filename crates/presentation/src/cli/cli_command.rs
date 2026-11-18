use clap::Subcommand;

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    #[command(name = "choose")]
    Choose {
        #[arg(value_name = "NAME")]
        name: Option<String>,
    },
    #[command(name = "create-worktree", alias = "create")]
    CreateWorktree {
        #[arg(value_name = "BRANCH")]
        branch: Option<String>,
    },
    #[command(name = "cleanup")]
    Cleanup,
    #[command(name = "settings")]
    Settings {
        #[command(subcommand)]
        command: Option<SettingsCommand>,
    },
    #[command(name = "tui")]
    Tui {
        #[command(subcommand)]
        command: Option<TuiCommand>,
    },
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum SettingsCommand {
    #[command(name = "list")]
    List,
    #[command(name = "get")]
    Get { name: String },
    #[command(name = "set")]
    Set { name: String, value: String },
}

#[derive(Subcommand, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuiCommand {
    #[command(name = "choose")]
    Choose,
    #[command(name = "cleanup")]
    Cleanup,
    #[command(name = "demo-loading")]
    DemoLoading,
}
