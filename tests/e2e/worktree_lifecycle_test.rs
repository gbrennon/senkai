mod tests {
    use std::env;
    use std::process::Command as ProcessCommand;

    use senkai_core::selector::SelectionResult;
    use senkai_presentation::cli::Cli;
    use senkai_presentation::cli::command::Command;
    use tempfile::tempdir;

    use super::super::common::fakes::git::FakeGitCreate;
    use super::super::common::fakes::loading::FakeLoading;
    use super::super::common::fakes::selector::FakeSelector;
    use super::super::common::fakes::tmux::FakeTmxTest;

    fn create_cli(repository: &std::path::Path) -> Cli {
        Cli::new(
            Box::new(FakeTmxTest::outside_tmux(&repository.to_string_lossy())),
            Box::new(FakeGitCreate),
            Box::new(FakeSelector::new(vec![SelectionResult::Cancelled])),
            Box::new(FakeLoading::new()),
        )
    }

    fn usage_output(arguments: &[&str]) -> String {
        let binary = env::var_os("CARGO_BIN_EXE_sk").expect("missing sk binary path");
        let output = ProcessCommand::new(binary)
            .args(arguments)
            .output()
            .expect("failed to run sk");
        assert!(output.status.success());
        String::from_utf8(output.stdout).expect("usage output was not UTF-8")
    }

    #[test]
    fn no_arguments_render_usage() {
        assert!(usage_output(&[]).contains("Usage: sk"));
    }

    #[test]
    fn short_help_renders_usage() {
        assert!(usage_output(&["-h"]).contains("Usage: sk"));
    }

    #[test]
    fn long_help_renders_usage() {
        assert!(usage_output(&["--help"]).contains("Usage: sk"));
    }

    #[test]
    fn parse_args_defaults_to_choose() {
        let repository = tempdir().unwrap();
        let cli = create_cli(repository.path());
        let (command, rest) = cli.parse_args(&["sk".to_string()]);

        assert_eq!(command, Command::Choose);
        assert!(rest.is_empty());
    }

    #[test]
    fn create_worktree_runs_through_application_entrypoint() {
        let repository = tempdir().unwrap();
        let target = repository.path().join(".worktrees").join("feature");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let previous_directory = env::current_dir().unwrap();
        let previous_root = env::var_os("SENKAI_ROOT");
        unsafe {
            env::set_var("SENKAI_ROOT", repository.path());
        }

        let result = create_cli(repository.path())
            .dispatch(Command::CreateWorktree, &["feature".to_string()]);

        assert!(result.is_ok());
        assert_eq!(env::current_dir().unwrap(), target);
        env::set_current_dir(previous_directory).unwrap();
        match previous_root {
            Some(value) => unsafe { env::set_var("SENKAI_ROOT", value) },
            None => unsafe { env::remove_var("SENKAI_ROOT") },
        }
    }
}
