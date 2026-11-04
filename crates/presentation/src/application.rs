use crate::cli::Cli;

pub struct Application {
    cli: Cli,
}

impl Application {
    pub fn new(cli: Cli) -> Self {
        Self { cli }
    }

    pub fn run(self, args: &[String]) {
        self.cli.run(args);
    }

    pub fn cli(&self) -> &Cli {
        &self.cli
    }
}
