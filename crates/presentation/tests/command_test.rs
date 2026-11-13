mod tests {
    use senkai_presentation::cli::command::Command;

    #[test]
    fn parse_choose() {
        assert_eq!(Command::parse("choose"), Command::Choose);
    }

    #[test]
    fn parse_tui_defaults_to_tui_choose() {
        assert_eq!(Command::parse("tui"), Command::TuiChoose);
    }

    #[test]
    fn parse_create_worktree() {
        assert_eq!(Command::parse("create-worktree"), Command::CreateWorktree);
    }

    #[test]
    fn parse_cleanup() {
        assert_eq!(Command::parse("cleanup"), Command::Cleanup);
    }

    #[test]
    fn parse_unknown_as_choose() {
        assert_eq!(Command::parse("bogus"), Command::Choose);
    }

    #[test]
    fn interactive_commands() {
        assert!(Command::Choose.is_interactive());
        assert!(Command::Cleanup.is_interactive());
        assert!(Command::TuiChoose.is_interactive());
        assert!(!Command::CreateWorktree.is_interactive());
    }

    #[test]
    fn tui_commands_are_distinct_from_plain_commands() {
        assert!(Command::TuiChoose.is_tui());
        assert!(Command::TuiCleanup.is_tui());
        assert!(!Command::Choose.is_tui());
    }
}
