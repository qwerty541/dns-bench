use clap::Parser;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::convert::From;
use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::time::Duration;
use std::time::Instant;
use tabled::Table;
use tabled::Tabled;
use trust_dns_resolver::config::NameServerConfig;
use trust_dns_resolver::config::Protocol;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::Resolver;

#[derive(Debug, Clone, Parser)]
#[command(next_line_help = true)]
#[command(author, version, about, long_about = None)]

struct Arguments {
    #[arg(long, default_value = "google.com")]
    domain: String,
}

#[derive(Debug, Clone)]
struct DnsEntry {
    name: String,
    socker_addr: SocketAddr,
}

lazy_static::lazy_static! {
    static ref DNS_ENTRIES: Vec<DnsEntry> = vec![
        DnsEntry { name: String::from("Google"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53) },
        DnsEntry { name: String::from("Google"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4)), 53) },
        DnsEntry { name: String::from("Cloudflare"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 53) },
        DnsEntry { name: String::from("Cloudflare"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)), 53) },
        DnsEntry { name: String::from("Quad9"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9)), 53) },
        DnsEntry { name: String::from("Quad9"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(149, 112, 112, 112)), 53) },
        DnsEntry { name: String::from("Provider"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 53) },
        DnsEntry { name: String::from("Control D"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(76, 76, 2, 0)), 53) },
        DnsEntry { name: String::from("Control D"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(76, 76, 10, 0)), 53) },
        DnsEntry { name: String::from("OpenDNS Home"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(208, 67, 222, 222)), 53) },
        DnsEntry { name: String::from("OpenDNS Home"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(208, 67, 220, 220)), 53) },
        DnsEntry { name: String::from("CleanBrowsing"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(185, 228, 168, 9)), 53) },
        DnsEntry { name: String::from("CleanBrowsing"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(185, 228, 169, 9)), 53) },
        DnsEntry { name: String::from("Alternate DNS"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(76, 76, 19, 19)), 53) },
        DnsEntry { name: String::from("Alternate DNS"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(76, 223, 122, 150)), 53) },
        DnsEntry { name: String::from("AdGuard DNS"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(94, 140, 14, 14)), 53) },
        DnsEntry { name: String::from("AdGuard DNS"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(94, 140, 15, 15)), 53) },
    ];
}

#[derive(Debug, Clone)]
enum TimeResult {
    Succeeded(Duration),
    Failed(String),
}

impl fmt::Display for TimeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeResult::Succeeded(duration) => write!(f, "{:?}", duration),
            TimeResult::Failed(error) => write!(f, "{}", error),
        }
    }
}

#[derive(Debug, Clone, Tabled)]
struct ResultEntry {
    name: String,
    ip: IpAddr,
    resolved_ip: IpAddr,
    time: TimeResult,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut result_entries: Vec<ResultEntry> = Vec::new();

    println!(
        "Starting DNS benchmark with test domain: {}...",
        arguments.domain
    );

    // Create a progress bar with the desired style
    let pb = ProgressBar::new(DNS_ENTRIES.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:60.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    'dns_bench: for dns_entry in DNS_ENTRIES.iter() {
        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(NameServerConfig {
            socket_addr: dns_entry.socker_addr,
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_negative_responses: false,
            bind_addr: None,
        });
        let resolver_opts = ResolverOpts::default();
        let resolver = Resolver::new(resolver_config, resolver_opts)?;

        // Measure the DNS resolution time
        let start_time = Instant::now();
        let Ok(response) = resolver.lookup_ip(arguments.domain.clone()) else {
            let result_entry = ResultEntry {
                name: dns_entry.name.clone(),
                ip: dns_entry.socker_addr.ip(),
                resolved_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                time: TimeResult::Failed(String::from("Failed to resolve")),
            };
            result_entries.push(result_entry);
            pb.inc(1);
            continue 'dns_bench;
        };

        // Calculate the elapsed time
        let elapsed_time = start_time.elapsed();

        // Create the result entry
        let result_entry = ResultEntry {
            name: dns_entry.name.clone(),
            ip: dns_entry.socker_addr.ip(),
            resolved_ip: response.iter().next().unwrap(),
            time: TimeResult::Succeeded(elapsed_time),
        };
        result_entries.push(result_entry);
        pb.inc(1);
    }

    pb.finish_and_clear();

    // Sort result entries by time
    result_entries.sort_by(|a, b| match (a.time.clone(), b.time.clone()) {
        (TimeResult::Succeeded(a), TimeResult::Succeeded(b)) => a.cmp(&b),
        (TimeResult::Succeeded(_), TimeResult::Failed(_)) => std::cmp::Ordering::Less,
        (TimeResult::Failed(_), TimeResult::Succeeded(_)) => std::cmp::Ordering::Greater,
        (TimeResult::Failed(_), TimeResult::Failed(_)) => std::cmp::Ordering::Equal,
    });

    // Print the result
    let table = Table::new(&result_entries).to_string();
    println!("{}", table);

    Ok(())
}
