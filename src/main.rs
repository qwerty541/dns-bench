mod app;
mod args;
mod config;
mod result;
mod servers;

use app::DnsBenchApplication;
use args::Arguments;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut app = DnsBenchApplication::new(arguments);
    app.run();

    Ok(())
}
