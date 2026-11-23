mod tests {

    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use std::rc::Rc;

    use senkai_core::selector::SelectionResult;
    use senkai_presentation::cli::Cli;
    use senkai_presentation::cli::command::Command;
    use tempfile::TempDir;

    use super::super::common::fakes::git::{FakeGitCreate, FakeGitForChoose};
    use super::super::common::fakes::loading::FakeLoading;
    use super::super::common::fakes::selector::FakeSelector;
    use super::super::common::fakes::tmux::FakeTmxTest;

    fn create_cli(
        terminal: FakeTmxTest,
        repository: impl senkai_core::ports::RepositoryPort + 'static,
        selection: SelectionResult,
    ) -> Cli {
        Cli::new(
            Box::new(terminal),
            Box::new(repository),
            Box::new(FakeSelector::new(vec![selection])),
            Box::new(FakeLoading::new()),
        )
    }

    fn existing_worktree() -> (TempDir, std::path::PathBuf) {
        let repository = tempfile::tempdir().unwrap();
        let target = repository.path().join(".worktrees").join("feature");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        (repository, target)
    }

    fn choose_repository(repository: &Path, target: &Path) -> FakeGitForChoose {
        FakeGitForChoose::existing_worktree(
            &repository.to_string_lossy(),
            &target.to_string_lossy(),
            "feat-a",
        )
    }

    fn create_parse_cli(repository: &Path) -> Cli {
        create_cli(
            FakeTmxTest::outside_tmux(&repository.to_string_lossy()),
            FakeGitCreate,
            SelectionResult::Cancelled,
        )
    }

    #[test]
    fn parse_args_defaults_to_choose() {
        let repository = tempfile::tempdir().unwrap();
        let cli = create_parse_cli(repository.path());
        let (command, rest) = cli.parse_args(&["sk".to_string()]);

        assert_eq!(command, Command::Choose);
        assert!(rest.is_empty());
    }

    #[test]
    fn outside_print_path_resolves_existing_worktree_without_window_action() {
        let (repository, target) = existing_worktree();
        let terminal = FakeTmxTest::outside_tmux(&repository.path().to_string_lossy());
        let selected_window = Rc::clone(&terminal.selected_window);
        let cli = create_cli(
            terminal,
            choose_repository(repository.path(), &target),
            SelectionResult::Selected(0),
        );

        cli.dispatch(Command::ChoosePath, &[]).unwrap();

        assert!(selected_window.borrow().is_none());
    }

    #[test]
    fn inside_plain_switch_selects_window_with_resolved_worktree() {
        let (repository, target) = existing_worktree();
        let terminal = FakeTmxTest::new(&repository.path().to_string_lossy());
        let selected_window = Rc::clone(&terminal.selected_window);
        let cli = create_cli(
            terminal,
            choose_repository(repository.path(), &target),
            SelectionResult::Selected(0),
        );

        cli.dispatch(Command::Choose, &[]).unwrap();

        assert_eq!(
            selected_window.borrow().as_ref(),
            Some(&(
                "feat-a".to_string(),
                target.to_string_lossy().into_owned(),
                "/bin/bash".to_string()
            ))
        );
    }

    #[test]
    fn inside_print_path_does_not_select_window() {
        let (repository, target) = existing_worktree();
        let terminal = FakeTmxTest::new(&repository.path().to_string_lossy());
        let selected_window = Rc::clone(&terminal.selected_window);
        let cli = create_cli(
            terminal,
            choose_repository(repository.path(), &target),
            SelectionResult::Selected(0),
        );

        cli.dispatch(Command::ChoosePath, &[]).unwrap();

        assert!(selected_window.borrow().is_none());
    }

    #[test]
    fn outside_plain_switch_runs_injected_shell_at_worktree() {
        let (repository, target) = existing_worktree();
        let shell = repository.path().join("test-shell");
        let shell_pwd = repository.path().join("shell-pwd");
        std::fs::write(
            &shell,
            format!("#!/bin/sh\npwd > '{}'\n", shell_pwd.display()),
        )
        .unwrap();
        std::fs::set_permissions(&shell, std::fs::Permissions::from_mode(0o755)).unwrap();

        let mut terminal = FakeTmxTest::outside_tmux(&repository.path().to_string_lossy());
        terminal.shell_cmd = shell.to_string_lossy().into_owned();
        let cli = create_cli(
            terminal,
            choose_repository(repository.path(), &target),
            SelectionResult::Selected(0),
        );

        cli.dispatch(Command::Choose, &[]).unwrap();

        assert_eq!(
            std::fs::read_to_string(shell_pwd).unwrap().trim(),
            target.to_string_lossy()
        );
    }
}
