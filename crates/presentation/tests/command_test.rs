mod tests {
    use senkai_presentation::cli::command::Command;

    #[test]
    fn interactive_commands() {
        assert!(Command::Choose.is_interactive());
        assert!(Command::ChoosePath.is_interactive());
        assert!(Command::Cleanup.is_interactive());
        assert!(Command::TuiChoose.is_interactive());
    }

    #[test]
    fn tui_commands_are_distinct_from_plain_commands() {
        assert!(Command::TuiChoose.is_tui());
        assert!(Command::TuiCleanup.is_tui());
        assert!(!Command::Choose.is_tui());
        assert!(!Command::ChoosePath.is_tui());
    }
}
