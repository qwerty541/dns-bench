use clap::builder::PossibleValue;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl ValueEnum for IpAddr {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::V4, Self::V6]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::V4 => PossibleValue::new("v4"),
            Self::V6 => PossibleValue::new("v6"),
        })
    }
}

argument_impl_from_str!(IpAddr);
argument_impl_display!(IpAddr);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl ValueEnum for Protocol {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Tcp, Self::Udp]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Tcp => PossibleValue::new("tcp"),
            Self::Udp => PossibleValue::new("udp"),
        })
    }
}

argument_impl_from_str!(Protocol);
argument_impl_display!(Protocol);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl ValueEnum for Style {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Empty,
            Self::Blank,
            Self::Ascii,
            Self::Psql,
            Self::Markdown,
            Self::Modern,
            Self::Sharp,
            Self::Rounded,
            Self::ModernRounded,
            Self::Extended,
            Self::Dots,
            Self::ReStructuredText,
            Self::AsciiRounded,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Empty => PossibleValue::new("empty"),
            Self::Blank => PossibleValue::new("blank"),
            Self::Ascii => PossibleValue::new("ascii"),
            Self::Psql => PossibleValue::new("psql"),
            Self::Markdown => PossibleValue::new("markdown"),
            Self::Modern => PossibleValue::new("modern"),
            Self::Sharp => PossibleValue::new("sharp"),
            Self::Rounded => PossibleValue::new("rounded"),
            Self::ModernRounded => PossibleValue::new("modern_rounded"),
            Self::Extended => PossibleValue::new("extended"),
            Self::Dots => PossibleValue::new("dots"),
            Self::ReStructuredText => PossibleValue::new("re_structured_text"),
            Self::AsciiRounded => PossibleValue::new("ascii_rounded"),
        })
    }
}

argument_impl_from_str!(Style);
argument_impl_display!(Style);
