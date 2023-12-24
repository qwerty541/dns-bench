use clap::Parser;
use hickory_resolver::config::NameServerConfig;
use hickory_resolver::config::Protocol;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::config::ResolverOpts;
use hickory_resolver::Resolver;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections;
use std::convert::From;
use std::convert::Into;
use std::fmt;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::sync;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tabled::Table;
use tabled::Tabled;

#[derive(Debug, Clone, Parser)]
#[command(next_line_help = true)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// The domain to resolve.
    #[arg(long, default_value = "google.com")]
    domain: String,
    /// The number of threads to use.
    #[arg(long, default_value = "8")]
    threads: usize,
    /// The number of requests to make.
    #[arg(long, default_value = "3")]
    requests: usize,
    /// The timeout in seconds.
    #[arg(long, default_value = "3")]
    timeout: u64,
}

#[derive(Debug, Clone)]
struct DnsEntry {
    name: String,
    socker_addr: SocketAddr,
}

macro_rules! dns_entry {
    ($name:expr, $ip:expr, $port:expr) => {
        DnsEntry {
            name: String::from($name),
            socker_addr: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new($ip.0, $ip.1, $ip.2, $ip.3)),
                $port,
            ),
        }
    };
}

lazy_static::lazy_static! {
    static ref DNS_ENTRIES: Vec<DnsEntry> = vec![
        dns_entry!("Google", (8, 8, 8, 8), 53),
        dns_entry!("Google", (8, 8, 4, 4), 53),
        dns_entry!("Cloudflare", (1, 1, 1, 1), 53),
        dns_entry!("Cloudflare", (1, 0, 0, 1), 53),
        dns_entry!("Quad9", (9, 9, 9, 9), 53),
        dns_entry!("Quad9", (149, 112, 112, 112), 53),
        dns_entry!("Provider", (192, 168, 0, 1), 53),
        dns_entry!("Control D", (76, 76, 2, 0), 53),
        dns_entry!("Control D", (76, 76, 10, 0), 53),
        dns_entry!("OpenDNS Home", (208, 67, 222, 222), 53),
        dns_entry!("OpenDNS Home", (208, 67, 220, 220), 53),
        dns_entry!("CleanBrowsing", (185, 228, 168, 9), 53),
        dns_entry!("CleanBrowsing", (185, 228, 169, 9), 53),
        dns_entry!("AdGuard DNS", (94, 140, 14, 14), 53),
        dns_entry!("AdGuard DNS", (94, 140, 15, 15), 53),
        dns_entry!("Comodo Secure DNS", (8, 26, 56, 26), 53),
        dns_entry!("Comodo Secure DNS", (8, 20, 247, 20), 53),
        dns_entry!("Level3", (209, 244, 0, 3), 53),
        dns_entry!("Level3", (209, 244, 0, 4), 53),
        dns_entry!("Verisign", (64, 6, 64, 6), 53),
        dns_entry!("Verisign", (64, 6, 65, 6), 53),
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

#[derive(Debug, Clone)]
struct MeasureResult {
    name: String,
    ip: IpAddr,
    resolved_ip: IpAddr,
    time: TimeResult,
}

#[derive(Debug, Clone, Tabled)]
struct ResultEntry {
    name: String,
    ip: IpAddr,
    last_resolved_ip: IpAddr,
    /// String with the following format: "successfull_requests/total_requests (success_rate))"
    successfull_requests: String,
    first_duration: TimeResult,
    average_duration: TimeResult,
}

impl From<Vec<MeasureResult>> for ResultEntry {
    fn from(value: Vec<MeasureResult>) -> Self {
        let mut successfull_requests = 0;
        let mut total_time = Duration::new(0, 0);
        let mut last_resolved_ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        for measure_result in &value {
            match measure_result.time {
                TimeResult::Succeeded(duration) => {
                    successfull_requests += 1;
                    total_time += duration;
                    last_resolved_ip = measure_result.resolved_ip;
                }
                TimeResult::Failed(_) => {}
            }
        }

        let average_duration = if successfull_requests > 0 {
            TimeResult::Succeeded(total_time / successfull_requests as u32)
        } else {
            TimeResult::Failed(String::from("No successfull requests"))
        };

        ResultEntry {
            name: value[0].name.clone(),
            ip: value[0].ip,
            last_resolved_ip,
            successfull_requests: format!(
                "{}/{} ({:.2}%)",
                successfull_requests,
                value.len(),
                successfull_requests as f32 / value.len() as f32 * 100.0
            ),
            first_duration: value[0].time.clone(),
            average_duration,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();

    println!(
        "Starting DNS benchmark with the following parameters:\n\
        Domain: {}\n\
        Threads: {}\n\
        Requests: {}",
        arguments.domain, arguments.threads, arguments.requests
    );

    // Create a progress bar with the desired style
    let pb = ProgressBar::new((DNS_ENTRIES.len() * arguments.requests) as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.bold.green} [{elapsed}] [{bar:80.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    // Create the shared DNS entries and result entries
    let dns_entries = sync::Arc::new(sync::Mutex::new(collections::VecDeque::from(
        DNS_ENTRIES.clone(),
    )));
    let result_entries = sync::Arc::new(sync::Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    let bench_start_time = Instant::now();

    // Create the threads
    for _ in 0..arguments.threads {
        let dns_entries_clone = dns_entries.clone();
        let result_entries_clone = result_entries.clone();
        let arguments_clone = arguments.clone();
        let pb_clone = pb.clone();

        handles.push(thread::spawn(move || loop {
            let dns_entry = {
                let mut dns_entries_clone = dns_entries_clone.lock().unwrap();
                dns_entries_clone.pop_front()
            };

            if let Some(dns_entry) = dns_entry {
                let mut measure_results = Vec::new();

                for _ in 0..arguments_clone.requests {
                    // Create a new resolver for each request to avoid caching
                    let mut resolver_config = ResolverConfig::new();
                    resolver_config.add_name_server(NameServerConfig {
                        socket_addr: dns_entry.socker_addr,
                        protocol: Protocol::Udp,
                        tls_dns_name: None,
                        trust_negative_responses: false,
                        bind_addr: None,
                    });
                    let mut resolver_opts = ResolverOpts::default();
                    resolver_opts.attempts = 0;
                    resolver_opts.timeout = Duration::from_secs(arguments_clone.timeout);
                    let resolver = Resolver::new(resolver_config, resolver_opts).unwrap();

                    // Measure the time it takes to resolve the domain
                    let start_time = Instant::now();
                    let result_entry: MeasureResult =
                        match resolver.lookup_ip(arguments_clone.domain.clone()) {
                            Ok(response) => {
                                let elapsed_time = start_time.elapsed();
                                MeasureResult {
                                    name: dns_entry.name.clone(),
                                    ip: dns_entry.socker_addr.ip(),
                                    resolved_ip: response.iter().next().unwrap(),
                                    time: TimeResult::Succeeded(elapsed_time),
                                }
                            }
                            Err(e) => MeasureResult {
                                name: dns_entry.name.clone(),
                                ip: dns_entry.socker_addr.ip(),
                                resolved_ip: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                                time: TimeResult::Failed(e.to_string()),
                            },
                        };
                    measure_results.push(result_entry);
                    pb_clone.inc(1);
                }
                let result_entry: ResultEntry = measure_results.into();
                result_entries_clone.lock().unwrap().push(result_entry);
            } else {
                break;
            }
        }));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    pb.finish_and_clear();

    // Sort result entries by average duration, failed entries are at the end.
    let mut result_entries = result_entries.lock().unwrap();
    result_entries.sort_by(|a, b| {
        let a = match a.average_duration {
            TimeResult::Succeeded(duration) => duration,
            TimeResult::Failed(_) => Duration::new(u64::MAX, 0),
        };
        let b = match b.average_duration {
            TimeResult::Succeeded(duration) => duration,
            TimeResult::Failed(_) => Duration::new(u64::MAX, 0),
        };
        a.cmp(&b)
    });

    // Print the result
    let table = Table::new(&*result_entries).to_string();
    println!("{}", table);

    // Print the benchmark time
    let bench_elapsed_time = bench_start_time.elapsed();
    println!("Benchmark completed in {bench_elapsed_time:?}",);

    Ok(())
}
