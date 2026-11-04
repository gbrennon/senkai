use senkai_presentation::{Application, CompositionRoot};

mod common;
use common::fakes::git::StubGit;
use common::fakes::tmux::StubTmx;

pub struct CompositionRootProbe;

impl CompositionRootProbe {
    pub fn probe() -> Application {
        CompositionRoot::compose(Box::new(StubTmx), Box::new(StubGit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_root_composes_application() {
        let app = CompositionRootProbe::probe();
        let (cmd, rest) = app.cli().parse_args(&["tmux-worktrees".to_string()]);
        assert_eq!(cmd, senkai_presentation::cli::command::Command::Choose);
        assert!(rest.is_empty());
    }
}
