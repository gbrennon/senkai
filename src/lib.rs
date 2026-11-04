pub use tmux_worktrees_infrastructure::Container;
pub use tmux_worktrees_presentation::{Application, CompositionRoot};

pub fn build_app() -> Application {
    CompositionRoot::compose(Container::build_tmux(), Container::build_repository())
}
