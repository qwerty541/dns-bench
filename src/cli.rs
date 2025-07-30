use crate::args::Format;
use crate::args::IpAddr;
use crate::args::Protocol;
use crate::args::Style;

use clap::Args;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

const HELP_TEMPLATE: &str = "\
{before-help}{name} {version}

{about}

Author: {author}
Source: https://github.com/qwerty541/dns-bench

{usage-heading} {usage}

{all-args}{after-help}
";

#[derive(Debug, Clone, Parser)]
#[command(next_line_help = true)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE,
)]
pub struct Cli {
    #[command(flatten)]
    pub args: DefaultArgs,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Clone, Args)]
pub struct DefaultArgs {
    #[command(flatten)]
    pub args: SharedArgs,

    /// Save the configurations to a file in users home directory.
    #[arg(long)]
    pub save_config: bool,
}

#[derive(Debug, Clone, Args)]
pub struct SharedArgs {
    /// The domain to resolve.
    #[arg(long)]
    pub domain: Option<String>,
    /// The number of threads to use.
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..256))]
    pub threads: Option<u16>,
    /// The number of requests to make.
    #[arg(long, value_parser = clap::value_parser!(u16).range(1..1000))]
    pub requests: Option<u16>,
    /// The timeout in seconds.
    #[arg(long, value_parser = clap::value_parser!(u64).range(1..60))]
    pub timeout: Option<u64>,
    /// The protocol to use.
    #[arg(long)]
    pub protocol: Option<Protocol>,
    /// The IP version to use for the name servers.
    #[arg(long)]
    pub name_servers_ip: Option<IpAddr>,
    /// The IP version to use for the lookup.
    #[arg(long)]
    pub lookup_ip: Option<IpAddr>,
    /// The style to use for the table.
    #[arg(long)]
    pub style: Option<Style>,
    /// Provide a custom list of servers to use instead of the default ones.
    #[arg(long)]
    pub custom_servers_file: Option<PathBuf>,
    /// The output format.
    #[arg(long)]
    pub format: Option<Format>,
    /// Skip autodetection of system DNS servers.
    #[arg(long)]
    pub skip_system_servers: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Commands related to configuration management.
    #[command(subcommand)]
    Config(ConfigCommand),
}

#[derive(Debug, Clone, Subcommand)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE
)]
pub enum ConfigCommand {
    /// Create a config file with default values if it does not exist.
    Init(ConfigInitArgs),
    /// Set one or more config values.
    Set(ConfigSetArgs),
    /// Reset config file to default values.
    Reset(ConfigResetArgs),
    /// Delete config file.
    Delete(ConfigDeleteArgs),
}

#[derive(Debug, Clone, Args)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE
)]
pub struct ConfigInitArgs;

#[derive(Debug, Clone, Args)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE
)]
pub struct ConfigSetArgs {
    #[command(flatten)]
    pub common: SharedArgs,
}

#[derive(Debug, Clone, Args)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE
)]
pub struct ConfigResetArgs;

#[derive(Debug, Clone, Args)]
#[command(
    author = clap::crate_authors!("\n"),
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE
)]
pub struct ConfigDeleteArgs;
