mod args;
mod result;
mod servers;

use args::Arguments;
use result::MeasureResult;
use result::ResultEntry;
use result::TimeResult;
use servers::IPV4_DNS_ENTRIES;
use servers::IPV6_DNS_ENTRIES;

use clap::Parser;
use hickory_resolver::config::NameServerConfig;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::config::ResolverOpts;
use hickory_resolver::Resolver;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::sync;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tabled::Table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();

    println!(
        "Starting DNS benchmark with the following parameters:\n\
        Domain: {}\n\
        Threads: {}\n\
        Requests: {}\n\
        Protocol: {}\n\
        Name servers: IP{}; Lookup: IP{}",
        arguments.domain,
        arguments.threads,
        arguments.requests,
        arguments.protocol,
        arguments.name_servers_ip,
        arguments.lookup_ip,
    );

    // Create a progress bar with the desired style
    let pb = ProgressBar::new(match arguments.name_servers_ip {
        args::IpAddr::V4 => IPV4_DNS_ENTRIES.len() * arguments.requests,
        args::IpAddr::V6 => IPV6_DNS_ENTRIES.len() * arguments.requests,
    } as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.bold.green} [{elapsed}] [{bar:80.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("#>-"),
    );

    // Create the shared DNS entries and result entries
    let dns_entries = sync::Arc::new(sync::Mutex::new(collections::VecDeque::from(
        match arguments.name_servers_ip {
            args::IpAddr::V4 => IPV4_DNS_ENTRIES.clone(),
            args::IpAddr::V6 => IPV6_DNS_ENTRIES.clone(),
        },
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
                        protocol: arguments_clone.protocol.into(),
                        tls_dns_name: None,
                        trust_negative_responses: false,
                        bind_addr: None,
                    });
                    let mut resolver_opts = ResolverOpts::default();
                    resolver_opts.attempts = 0;
                    resolver_opts.timeout = Duration::from_secs(arguments_clone.timeout);
                    resolver_opts.ip_strategy = arguments_clone.lookup_ip.into();
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
                                resolved_ip: match arguments_clone.lookup_ip {
                                    args::IpAddr::V4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                                    args::IpAddr::V6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                                },
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
