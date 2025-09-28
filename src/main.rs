mod app;
mod args;
mod bench;
mod cli;
mod commands;
mod config;
mod custom;
mod gateway;
mod resolver;
mod result;
mod servers;
mod system;
#[cfg(test)]
mod test_utils;

use app::Application;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    Application::run(cli)?;
    Ok(())
}
