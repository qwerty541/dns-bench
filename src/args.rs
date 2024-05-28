use clap::Parser;
use clap::ValueEnum;
use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::config::Protocol as ResolverProtocol;
use std::fmt;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Parser)]
#[command(next_line_help = true)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The domain to resolve.
    #[arg(long)]
    pub domain: Option<String>,
    /// The number of threads to use.
    #[arg(long)]
    pub threads: Option<usize>,
    /// The number of requests to make.
    #[arg(long)]
    pub requests: Option<usize>,
    /// The timeout in seconds.
    #[arg(long)]
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
    /// Save the configurations to a file in users home directory.
    #[arg(long)]
    pub save_config: bool,
    /// Provide a custom list of servers to use instead of the default ones.
    #[arg(long)]
    pub custom_servers_file: Option<PathBuf>,
}

macro_rules! argument_impl_from_str {
    ($type:ty) => {
        impl FromStr for $type {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                for variant in Self::value_variants() {
                    if variant.to_possible_value().unwrap().matches(s, false) {
                        return Ok(*variant);
                    }
                }
                Err(format!("Invalid variant: {}", s))
            }
        }
    };
}

macro_rules! argument_impl_display {
    ($type:ty) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.to_possible_value()
                    .expect("no values are skipped")
                    .get_name()
                    .fmt(f)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum IpAddr {
    V4,
    V6,
}

impl From<IpAddr> for LookupIpStrategy {
    fn from(val: IpAddr) -> Self {
        match val {
            IpAddr::V4 => LookupIpStrategy::Ipv4Only,
            IpAddr::V6 => LookupIpStrategy::Ipv6Only,
        }
    }
}

argument_impl_from_str!(IpAddr);
argument_impl_display!(IpAddr);

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
}

impl From<Protocol> for ResolverProtocol {
    fn from(val: Protocol) -> Self {
        match val {
            Protocol::Tcp => ResolverProtocol::Tcp,
            Protocol::Udp => ResolverProtocol::Udp,
        }
    }
}

argument_impl_from_str!(Protocol);
argument_impl_display!(Protocol);

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Empty,
    Blank,
    Ascii,
    Psql,
    Markdown,
    Modern,
    Sharp,
    Rounded,
    ModernRounded,
    Extended,
    Dots,
    ReStructuredText,
    AsciiRounded,
}

argument_impl_from_str!(Style);
argument_impl_display!(Style);
