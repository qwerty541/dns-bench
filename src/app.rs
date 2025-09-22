use crate::cli::Cli;
use crate::cli::Commands;
use crate::cli::ConfigCommand;
use crate::commands::*;

#[derive(Debug, Clone)]
pub struct Application;

impl Application {
    pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
        match cli.command {
            Some(Commands::Config(ConfigCommand::Init(_))) => ConfigInitCommand.run(()),
            Some(Commands::Config(ConfigCommand::Set(set_args))) => ConfigSetCommand.run(set_args),
            Some(Commands::Config(ConfigCommand::Reset(_))) => ConfigResetCommand.run(()),
            Some(Commands::Config(ConfigCommand::Delete(_))) => ConfigDeleteCommand.run(()),
            None => BenchmarkRunnerCommand.run(cli.args),
        }
    }
}
