use senkai_infrastructure::Container;

pub struct ContainerProbe;

impl ContainerProbe {
    pub fn probe_multiplexer() -> String {
        let mux = Container::build_multiplexer();
        mux.resolve_worktree_dir()
    }

    pub fn probe_repository() -> bool {
        let _repo = Container::build_repository();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_builds_multiplexer_port_without_panicking() {
        assert_eq!(ContainerProbe::probe_multiplexer(), ".worktrees");
    }

    #[test]
    fn container_builds_repository_port_without_panicking() {
        assert!(ContainerProbe::probe_repository());
    }
}
