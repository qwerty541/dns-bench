use clap::Parser;
use clap::ValueEnum;
use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::config::Protocol as ResolverProtocol;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Parser)]
#[command(next_line_help = true)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The domain to resolve.
    #[arg(long, default_value = "google.com")]
    pub domain: String,
    /// The number of threads to use.
    #[arg(long, default_value = "8")]
    pub threads: usize,
    /// The number of requests to make.
    #[arg(long, default_value = "3")]
    pub requests: usize,
    /// The timeout in seconds.
    #[arg(long, default_value = "3")]
    pub timeout: u64,
    /// The protocol to use.
    #[arg(long, default_value = "udp")]
    pub protocol: Protocol,
    /// The IP version to use for the name servers.
    #[arg(long, default_value = "v4")]
    pub name_servers_ip: IpAddr,
    /// The IP version to use for the lookup.
    #[arg(long, default_value = "v4")]
    pub lookup_ip: IpAddr,
    /// The style to use for the table.
    #[arg(long, default_value = "ascii")]
    pub style: Style,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum)]
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
