pub use senkai_infrastructure::Container;
pub use senkai_presentation::{Application, CompositionRoot};

pub fn build_app() -> Application {
    CompositionRoot::compose(Container::build_tmux(), Container::build_repository())
}
