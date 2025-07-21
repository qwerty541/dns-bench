mod app;
mod cli;
mod config;
mod custom;
mod result;
mod servers;
mod system;

use app::DnsBenchApplication;
use clap::Parser;
use cli::Cli;
use cli::Commands;
use cli::ConfigCommand;
use config::DnsBenchConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config(ConfigCommand::Init)) => {
            if DnsBenchConfig::config_file_exists()? {
                println!("Config file already exists.");
            } else {
                let config = DnsBenchConfig::default();
                config.write_into_file()?;
                println!("Config file created with default values.");
            }
        }
        Some(Commands::Config(ConfigCommand::Set(set_args))) => {
            if !DnsBenchConfig::config_file_exists()? {
                println!("Config file does not exist. Run `dns-bench config init` first.");
                return Ok(());
            }
            let mut config = DnsBenchConfig::try_load_from_file().unwrap_or_default();
            config.resolve_args(&set_args.common);
            config.write_into_file()?;
            println!("Config file updated.");
        }
        Some(Commands::Config(ConfigCommand::Reset)) => {
            if !DnsBenchConfig::config_file_exists()? {
                println!("Config file does not exist.");
            } else {
                let config = DnsBenchConfig::default();
                config.write_into_file()?;
                println!("Config file reset to default values.");
            }
        }
        Some(Commands::Config(ConfigCommand::Delete)) => {
            if !DnsBenchConfig::config_file_exists()? {
                println!("Config file does not exist.");
            } else {
                DnsBenchConfig::delete_config_file()?;
                println!("Config file deleted.");
            }
        }
        None => {
            let mut app = DnsBenchApplication::new(cli.args);
            app.run();
        }
    }

    Ok(())
}
