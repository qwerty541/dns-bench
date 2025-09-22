use crate::bench::BenchmarkRunner;
use crate::cli::DefaultArgs;
use crate::commands::Command;

#[derive(Debug, Clone)]
pub struct BenchmarkRunnerCommand;

impl Command<DefaultArgs> for BenchmarkRunnerCommand {
    fn run(&self, args: DefaultArgs) -> Result<(), Box<dyn std::error::Error>> {
        let mut bench = BenchmarkRunner::new(args);
        bench.run();
        Ok(())
    }
}
