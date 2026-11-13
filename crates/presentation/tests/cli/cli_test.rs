mod tests {
    use std::path::{Path, PathBuf};
    use std::rc::Rc;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    use super::super::common::fakes::git::*;
    use super::super::common::fakes::loading::*;
    use super::super::common::fakes::selector::*;
    use super::super::common::fakes::tmux::*;
    use senkai_core::ports::MultiplexerPort;
    use senkai_core::selector::SelectionResult;
    use senkai_presentation::cli::Cli;
    use senkai_presentation::cli::choose_worktree::ChooseWorktree;
    use senkai_presentation::cli::command::Command;
    use senkai_presentation::cli::repository_operations::RepositoryOperations;

    static PROCESS_STATE: OnceLock<Mutex<()>> = OnceLock::new();

    struct ProcessStateGuard {
        _lock: MutexGuard<'static, ()>,
        directory: PathBuf,
        root: Option<std::ffi::OsString>,
    }

    impl ProcessStateGuard {
        fn new(root: &Path) -> Self {
            let lock = PROCESS_STATE.get_or_init(|| Mutex::new(())).lock().unwrap();
            let directory = std::env::current_dir().unwrap();
            let previous_root = std::env::var_os("SENKAI_ROOT");
            unsafe {
                std::env::set_var("SENKAI_ROOT", root);
            }
            Self {
                _lock: lock,
                directory,
                root: previous_root,
            }
        }
    }

    impl Drop for ProcessStateGuard {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.directory).unwrap();
            match &self.root {
                Some(value) => unsafe { std::env::set_var("SENKAI_ROOT", value) },
                None => unsafe { std::env::remove_var("SENKAI_ROOT") },
            }
        }
    }

    fn repository_operations(
        terminal: impl MultiplexerPort + 'static,
        repository: impl senkai_core::ports::RepositoryPort + 'static,
    ) -> RepositoryOperations {
        RepositoryOperations::new(
            Rc::new(repository),
            Rc::new(terminal),
            Rc::new(FakeLoading::new()),
        )
    }

    fn cli_default(
        terminal: impl MultiplexerPort + 'static,
        repository: impl senkai_core::ports::RepositoryPort + 'static,
    ) -> Cli {
        Cli::new(
            Box::new(terminal),
            Box::new(repository),
            Box::new(StubSelector),
            Box::new(FakeLoading::new()),
        )
    }

    #[test]
    fn parse_args_defaults_to_choose() {
        let cli = cli_default(StubTmx, StubGit);
        let (command, rest) = cli.parse_args(&["sk".to_string()]);

        assert_eq!(command, Command::Choose);
        assert!(rest.is_empty());
    }

    #[test]
    fn parse_args_extracts_choose_name() {
        let cli = cli_default(StubTmx, StubGit);
        let (command, rest) = cli.parse_args(&[
            "sk".to_string(),
            "choose".to_string(),
            "feature/name".to_string(),
        ]);

        assert_eq!(command, Command::Choose);
        assert_eq!(rest, vec!["feature/name"]);
    }

    #[test]
    fn parse_args_extracts_create_branch() {
        let cli = cli_default(StubTmx, StubGit);
        let (command, rest) = cli.parse_args(&[
            "sk".to_string(),
            "create-worktree".to_string(),
            "feat/foo".to_string(),
        ]);

        assert_eq!(command, Command::CreateWorktree);
        assert_eq!(rest, vec!["feat/foo"]);
    }

    #[test]
    fn parse_args_maps_tui_commands() {
        let cli = cli_default(StubTmx, StubGit);

        assert_eq!(
            cli.parse_args(&["sk".to_string(), "tui".to_string()]).0,
            Command::TuiChoose
        );
        assert_eq!(
            cli.parse_args(&["sk".to_string(), "tui".to_string(), "cleanup".to_string(),])
                .0,
            Command::TuiCleanup
        );
        assert_eq!(
            cli.parse_args(&[
                "sk".to_string(),
                "tui".to_string(),
                "demo-loading".to_string(),
            ])
            .0,
            Command::TuiDemoLoading
        );
    }

    #[test]
    fn list_worktrees_returns_registered_external_paths() {
        let operations = repository_operations(
            StubTmx,
            FakeGitPorcelain::new(
                "worktree /tmp/repo\nworktree /tmp/repo/worktrees/feat-a\nworktree /tmp/external\n",
            ),
        );

        let paths = operations.list(Path::new("/tmp/repo")).unwrap();

        assert_eq!(
            paths,
            vec![
                Path::new("/tmp/repo/worktrees/feat-a"),
                Path::new("/tmp/external")
            ]
        );
    }

    #[test]
    fn create_worktree_propagates_repository_errors() {
        let operations = repository_operations(StubTmx, FakeGitWorktreeFail);

        let result = operations.create(
            Path::new("/tmp/repo"),
            Path::new("/tmp/target"),
            "feat/x",
            "main",
        );

        assert!(result.is_err());
    }

    #[test]
    fn worktree_branch_returns_question_mark_when_repository_fails() {
        let operations = repository_operations(StubTmx, FakeGitRevParseFail);

        let branch = operations.branch(Path::new("/tmp/worktree")).unwrap();

        assert_eq!(branch, "?");
    }

    #[test]
    fn remove_worktree_propagates_repository_errors() {
        let operations = repository_operations(FakeTmxNoWindow, FakeGitWorktreeRemoveFail);

        let result = operations.remove_and_delete(
            Path::new("/tmp/repo"),
            Path::new("/tmp/worktree"),
            "feat/x",
        );

        assert!(result.is_err());
    }

    #[test]
    fn dispatch_create_worktree_succeeds() {
        let cli = cli_default(FakeTmxCreateSuccess, FakeGitCreate);
        let result = cli.dispatch(Command::CreateWorktree, &["test-branch".to_string()]);

        assert!(result.is_ok());
    }

    #[test]
    fn selecting_an_existing_worktree_opens_its_registered_path() {
        let repository = tempfile::tempdir().unwrap();
        let target = repository.path().join(".worktrees").join("feat-a");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let _state = ProcessStateGuard::new(repository.path());
        let chooser = ChooseWorktree::new(
            Rc::new(FakeTmxTest::outside_tmux(
                &repository.path().to_string_lossy(),
            )),
            Rc::new(FakeGitForChoose::existing_worktree(
                &repository.path().to_string_lossy(),
                &target.to_string_lossy(),
                "",
            )),
            Rc::new(FakeSelector::new(vec![SelectionResult::Selected(0)])),
            Rc::new(FakeLoading::new()),
        );

        let result = chooser.run(None);

        assert!(result.is_ok());
        assert_eq!(std::env::current_dir().unwrap(), target);
    }

    #[test]
    fn tui_runs_inline_outside_tmux() {
        let repository = tempfile::tempdir().unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let _state = ProcessStateGuard::new(repository.path());
        let cli = Cli::new(
            Box::new(FakeTmxTest::outside_tmux(
                &repository.path().to_string_lossy(),
            )),
            Box::new(FakeGitPorcelain::new("")),
            Box::new(FakeSelector::new(vec![SelectionResult::Cancelled])),
            Box::new(FakeLoading::new()),
        );

        cli.run(&["sk".to_string(), "tui".to_string()]);
    }

    #[test]
    fn dispatch_choose_accepts_cancellation() {
        let repository = tempfile::tempdir().unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let _state = ProcessStateGuard::new(repository.path());
        let selector = FakeSelector::new(vec![SelectionResult::Cancelled]);
        let cli = Cli::new(
            Box::new(FakeTmxTest::new(&repository.path().to_string_lossy())),
            Box::new(FakeGitPorcelain::new("")),
            Box::new(selector),
            Box::new(FakeLoading::new()),
        );

        let result = cli.dispatch(Command::Choose, &[]);

        assert!(result.is_ok());
    }

    #[test]
    fn dispatch_cleanup_accepts_cancellation() {
        let repository = tempfile::tempdir().unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let _state = ProcessStateGuard::new(repository.path());
        let selector = FakeSelector::new(vec![SelectionResult::Cancelled]);
        let cli = Cli::new(
            Box::new(FakeTmxTest::new(&repository.path().to_string_lossy())),
            Box::new(FakeGitPorcelain::new("")),
            Box::new(selector),
            Box::new(FakeLoading::new()),
        );

        let result = cli.dispatch(Command::Cleanup, &[]);

        assert!(result.is_ok());
    }

    #[test]
    fn create_worktree_changes_directory_outside_tmux() {
        let repository = tempfile::tempdir().unwrap();
        let target = repository.path().join(".worktrees").join("new-feature");
        std::fs::create_dir_all(&target).unwrap();
        std::fs::create_dir(repository.path().join(".git")).unwrap();
        let _state = ProcessStateGuard::new(repository.path());

        let cli = cli_default(
            FakeTmxTest::outside_tmux(&repository.path().to_string_lossy()),
            FakeGitCreate,
        );
        let result = cli.dispatch(Command::CreateWorktree, &["new-feature".to_string()]);

        assert!(result.is_ok());
        assert_eq!(std::env::current_dir().unwrap(), target);
    }
}
