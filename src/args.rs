use clap::ValueEnum;
use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::config::Protocol as ResolverProtocol;
use std::fmt;
use std::str::FromStr;

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

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, ValueEnum, serde::Serialize, serde::Deserialize,
)]
pub enum Format {
    #[default]
    HumanReadable,
    Json,
    Xml,
    Csv,
}

argument_impl_from_str!(Format);
argument_impl_display!(Format);
