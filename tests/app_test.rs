pub struct AppProbe;

impl AppProbe {
    pub fn probe() -> tmux_worktrees::Application {
        tmux_worktrees::build_app()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_builds_and_runs_without_panicking() {
        let app = AppProbe::probe();
        let (cmd, rest) = app.cli().parse_args(&["tmux-worktrees".to_string()]);
        assert_eq!(
            cmd,
            tmux_worktrees_presentation::cli::command::Command::Choose
        );
        assert!(rest.is_empty());
    }
}
