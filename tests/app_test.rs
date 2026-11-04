pub struct AppProbe;

impl AppProbe {
    pub fn probe() -> senkai::Application {
        senkai::build_app()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_builds_and_runs_without_panicking() {
        let app = AppProbe::probe();
        let (cmd, rest) = app.cli().parse_args(&["sk".to_string()]);
        assert_eq!(cmd, senkai_presentation::cli::command::Command::Choose);
        assert!(rest.is_empty());
    }
}
