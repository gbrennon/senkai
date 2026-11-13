use crate::error::Result;

pub trait LoadingRunner {
    fn run_loading(&self, label: &str, job: Box<dyn FnOnce() -> Result<()> + '_>) -> Result<()>;
}
