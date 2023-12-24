use clap::builder::PossibleValue;
use clap::Parser;
use clap::ValueEnum;
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
}

#[derive(Debug, Clone, Copy)]
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

impl FromStr for Protocol {
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

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
