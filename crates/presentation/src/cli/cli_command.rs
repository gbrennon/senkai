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
    #[command(name = "tui")]
    Tui {
        #[command(subcommand)]
        command: Option<TuiCommand>,
    },
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
