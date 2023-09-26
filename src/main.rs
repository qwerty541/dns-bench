use std::time::Instant;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::config::NameServerConfig;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::Protocol;
use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::convert::From;
use tabled::Tabled;
use tabled::Table;
use std::time::Duration;
use std::fmt;

#[derive(Debug, Clone)]
struct DnsEntry {
    name: String,
    socker_addr: SocketAddr,
}

lazy_static::lazy_static! {
    static ref DNS_ENTRIES: Vec<DnsEntry> = vec![
        DnsEntry { name: String::from("Google"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53) },
        DnsEntry { name: String::from("Google 2"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 4, 4)), 53) },
        DnsEntry { name: String::from("Cloudflare"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 53) },
        DnsEntry { name: String::from("Cloudflare 2"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 0, 0, 1)), 53) },
        DnsEntry { name: String::from("Quad9"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(9, 9, 9, 9)), 53) },
        DnsEntry { name: String::from("Quad9 2"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(149, 112, 112, 112)), 53) },
        DnsEntry { name: String::from("Local"), socker_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)), 53) },
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
    let mut result_entries: Vec<ResultEntry> = Vec::new();

    'dns_bench: for dns_entry in DNS_ENTRIES.iter() {
        let mut resolver_config = ResolverConfig::new();
        resolver_config.add_name_server(NameServerConfig { 
            socket_addr: dns_entry.socker_addr,
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_negative_responses: false,
            bind_addr: None
        });
        let resolver_opts = ResolverOpts::default();
        let resolver = Resolver::new(resolver_config, resolver_opts)?;

        // Measure the DNS resolution time
        let start_time = Instant::now();
        let Ok(response) = resolver.lookup_ip("google.com") else {
            let result_entry = ResultEntry {
                name: dns_entry.name.clone(),
                ip: dns_entry.socker_addr.ip(),
                resolved_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                time: TimeResult::Failed(String::from("Failed to resolve")),
            };
            result_entries.push(result_entry);
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
    }

    // Sort result entries by time
    result_entries.sort_by(|a, b| {
        match (a.time.clone(), b.time.clone()) {
            (TimeResult::Succeeded(a), TimeResult::Succeeded(b)) => a.cmp(&b),
            (TimeResult::Succeeded(_), TimeResult::Failed(_)) => std::cmp::Ordering::Less,
            (TimeResult::Failed(_), TimeResult::Succeeded(_)) => std::cmp::Ordering::Greater,
            (TimeResult::Failed(_), TimeResult::Failed(_)) => std::cmp::Ordering::Equal,
        }
    });

    // Print the result
    let table = Table::new(&result_entries).to_string();
    println!("{}", table);

    Ok(())
}
