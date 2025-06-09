mod app;
mod args;
mod config;
mod custom;
mod result;
mod servers;
mod system;

use app::DnsBenchApplication;
use args::Arguments;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut app = DnsBenchApplication::new(arguments);
    app.run();

    Ok(())
}
