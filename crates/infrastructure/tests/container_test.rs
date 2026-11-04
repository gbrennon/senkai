use senkai_infrastructure::Container;

pub struct ContainerProbe;

impl ContainerProbe {
    pub fn probe_tmux() -> String {
        let tmux = Container::build_tmux();
        tmux.resolve_worktree_dir()
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
    fn container_builds_tmux_port_without_panicking() {
        assert_eq!(ContainerProbe::probe_tmux(), ".worktrees");
    }

    #[test]
    fn container_builds_repository_port_without_panicking() {
        assert!(ContainerProbe::probe_repository());
    }
}
