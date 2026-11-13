use senkai_core::error::Result;
use senkai_core::ports::LoadingRunner;

pub struct PlainLoading;

impl PlainLoading {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlainLoading {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadingRunner for PlainLoading {
    fn run_loading(&self, label: &str, job: Box<dyn FnOnce() -> Result<()> + '_>) -> Result<()> {
        println!("{label}");
        job()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_job_without_terminal_control() {
        let loading = PlainLoading::new();

        let result = loading.run_loading("Loading", Box::new(|| Ok(())));

        assert!(result.is_ok());
    }
}
