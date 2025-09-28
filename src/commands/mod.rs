mod bench;
mod config;

pub use bench::BenchmarkRunnerCommand;
pub use config::ConfigDeleteCommand;
pub use config::ConfigInitCommand;
pub use config::ConfigListCommand;
pub use config::ConfigResetCommand;
pub use config::ConfigSetCommand;

pub trait Command<A: clap::Args> {
    fn run(&self, args: A) -> Result<(), Box<dyn std::error::Error>>;
}
