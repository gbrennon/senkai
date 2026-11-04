pub mod container;

pub use container::Container;
pub use tmux_worktrees_presentation::cli::Cli;

pub fn build_cli() -> Cli {
    Container::build_cli()
}
