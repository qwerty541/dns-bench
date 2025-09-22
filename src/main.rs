mod app;
mod args;
mod bench;
mod cli;
mod commands;
mod config;
mod custom;
mod result;
mod servers;
mod system;

use app::Application;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    Application::run(cli)?;
    Ok(())
}
