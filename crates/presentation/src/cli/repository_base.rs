use std::rc::Rc;

use senkai_core::ports::RepositoryPort;

pub struct RepositoryBase {
    repository: Rc<dyn RepositoryPort>,
}

impl RepositoryBase {
    pub fn new(repository: Rc<dyn RepositoryPort>) -> Self {
        Self { repository }
    }

    pub fn resolve(&self, root: &std::path::Path, branch: &str) -> String {
        let remote = format!("refs/remotes/origin/{branch}");
        if self
            .repository
            .run_in(root, &["show-ref", "--verify", "--quiet", &remote])
            .map(|(status, _, _)| status == 0)
            .unwrap_or(false)
        {
            format!("origin/{branch}")
        } else {
            branch.to_string()
        }
    }
}
