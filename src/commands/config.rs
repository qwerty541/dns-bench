use crate::cli::ConfigSetArgs;
use crate::commands::Command;
use crate::config::DnsBenchConfig;

#[derive(Debug, Clone)]
pub struct ConfigInitCommand;

impl Command<()> for ConfigInitCommand {
    fn run(&self, _args: ()) -> Result<(), Box<dyn std::error::Error>> {
        if DnsBenchConfig::config_file_exists()? {
            println!("Config file already exists.");
        } else {
            let config = DnsBenchConfig::default();
            config.write_into_file()?;
            println!("Config file created with default values.");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConfigSetCommand;

impl Command<ConfigSetArgs> for ConfigSetCommand {
    fn run(&self, args: ConfigSetArgs) -> Result<(), Box<dyn std::error::Error>> {
        if !DnsBenchConfig::config_file_exists()? {
            println!("Config file does not exist. Run `dns-bench config init` first.");
        } else {
            let mut config = DnsBenchConfig::try_load_from_file().unwrap_or_default();
            config.resolve_args(&args.common);
            config.write_into_file()?;
            println!("Config file updated.");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConfigResetCommand;

impl Command<()> for ConfigResetCommand {
    fn run(&self, _args: ()) -> Result<(), Box<dyn std::error::Error>> {
        if !DnsBenchConfig::config_file_exists()? {
            println!("Config file does not exist.");
        } else {
            let config = DnsBenchConfig::default();
            config.write_into_file()?;
            println!("Config file reset to default values.");
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ConfigDeleteCommand;

impl Command<()> for ConfigDeleteCommand {
    fn run(&self, _args: ()) -> Result<(), Box<dyn std::error::Error>> {
        if !DnsBenchConfig::config_file_exists()? {
            println!("Config file does not exist.");
        } else {
            DnsBenchConfig::delete_config_file()?;
            println!("Config file deleted.");
        }

        Ok(())
    }
}
