mod tests {
    use std::env;
    use std::io::Write;
    use std::process::{Command as ProcessCommand, Stdio};

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
    fn run_git(repository: &std::path::Path, arguments: &[&str]) {
        let output = ProcessCommand::new("git")
            .args(arguments)
            .current_dir(repository)
            .output()
            .expect("failed to run git");
        assert!(
            output.status.success(),
            "git failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn print_path_outputs_selected_worktree_path_outside_tmux() {
        let repository = tempdir().unwrap();
        run_git(repository.path(), &["init", "-q"]);
        run_git(
            repository.path(),
            &["config", "user.email", "test@example.com"],
        );
        run_git(repository.path(), &["config", "user.name", "test"]);
        std::fs::write(repository.path().join("README.md"), "test\n").unwrap();
        run_git(repository.path(), &["add", "README.md"]);
        run_git(repository.path(), &["commit", "-qm", "initial"]);

        let target = repository.path().join(".worktrees").join("feature");
        run_git(
            repository.path(),
            &[
                "worktree",
                "add",
                "-q",
                ".worktrees/feature",
                "-b",
                "feature",
            ],
        );

        let binary = env::var_os("CARGO_BIN_EXE_sk").expect("missing sk binary path");
        let output = ProcessCommand::new(binary)
            .args(["choose", "--print-path", "feature"])
            .current_dir(repository.path())
            .env_remove("TMUX")
            .env_remove("TMUX_PANE")
            .env_remove("SENKAI_ROOT")
            .env_remove("TMUX_WORKTREES_ROOT")
            .output()
            .expect("failed to run sk");

        assert!(
            output.status.success(),
            "sk failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("{}\n", target.display())
        );
    }

    #[test]
    fn interactive_print_path_writes_only_selected_path_to_stdout() {
        let repository = tempdir().unwrap();
        run_git(repository.path(), &["init", "-q"]);
        run_git(
            repository.path(),
            &["config", "user.email", "test@example.com"],
        );
        run_git(repository.path(), &["config", "user.name", "test"]);
        std::fs::write(repository.path().join("README.md"), "test\n").unwrap();
        run_git(repository.path(), &["add", "README.md"]);
        run_git(repository.path(), &["commit", "-qm", "initial"]);

        let target = repository.path().join(".worktrees").join("feature");
        run_git(
            repository.path(),
            &[
                "worktree",
                "add",
                "-q",
                ".worktrees/feature",
                "-b",
                "feature",
            ],
        );

        let binary = env::var_os("CARGO_BIN_EXE_sk").expect("missing sk binary path");
        let mut child = ProcessCommand::new(binary)
            .args(["choose", "--print-path"])
            .current_dir(repository.path())
            .env_remove("TMUX")
            .env_remove("TMUX_PANE")
            .env_remove("SENKAI_ROOT")
            .env_remove("TMUX_WORKTREES_ROOT")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to run sk");
        child
            .stdin
            .take()
            .expect("missing child stdin")
            .write_all(b"1\n")
            .expect("failed to select worktree");
        let output = child.wait_with_output().expect("failed to wait for sk");

        assert!(
            output.status.success(),
            "sk failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("{}\n", target.display())
        );
        assert!(
            String::from_utf8(output.stderr)
                .unwrap()
                .contains("Worktree>")
        );
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
