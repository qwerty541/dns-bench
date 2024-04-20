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

pub struct DnsBenchApplication {
    arguments: Arguments,
    dns_entries: sync::Arc<sync::Mutex<collections::VecDeque<servers::DnsEntry>>>,
    result_entries: sync::Arc<sync::Mutex<Vec<ResultEntry>>>,
    threads: Vec<thread::JoinHandle<()>>,
    progress_bar: Option<ProgressBar>,
    bench_start_time: Option<Instant>,
}

impl DnsBenchApplication {
    /// Create a new instance of the application.
    pub fn new(arguments: Arguments) -> Self {
        Self {
            arguments,
            dns_entries: sync::Arc::new(sync::Mutex::new(collections::VecDeque::default())),
            result_entries: sync::Arc::new(sync::Mutex::new(Vec::new())),
            threads: Vec::new(),
            progress_bar: None,
            bench_start_time: None,
        }
    }

    /// Run the application.
    pub fn run(&mut self) {
        self.print_arguments_summary();
        self.init_progress_bar();
        self.fill_dns_entries();
        self.bench_start_time();
        self.spawn_threads();
        self.await_threads();
        self.clear_progress_bar();
        self.sort_result_entries();
        self.print_result();
        self.print_bench_elapsed_time();
    }

    /// Print the arguments summary.
    fn print_arguments_summary(&self) {
        println!(
            "Starting DNS benchmark with the following parameters:\n\
            Domain: {}\n\
            Threads: {}\n\
            Requests: {}\n\
            Protocol: {}\n\
            Name servers: IP{}; Lookup: IP{}",
            self.arguments.domain,
            self.arguments.threads,
            self.arguments.requests,
            self.arguments.protocol,
            self.arguments.name_servers_ip,
            self.arguments.lookup_ip,
        );
    }

    /// Create a progress bar with the desired style.
    fn init_progress_bar(&mut self) {
        let pb = ProgressBar::new(match self.arguments.name_servers_ip {
            args::IpAddr::V4 => IPV4_DNS_ENTRIES.len() * self.arguments.requests,
            args::IpAddr::V6 => IPV6_DNS_ENTRIES.len() * self.arguments.requests,
        } as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.bold.green} [{elapsed}] [{bar:80.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        self.progress_bar = Some(pb);
    }

    /// Fill the DNS entries with the desired IP version.
    fn fill_dns_entries(&mut self) {
        let dns_entries = match self.arguments.name_servers_ip {
            args::IpAddr::V4 => IPV4_DNS_ENTRIES.clone(),
            args::IpAddr::V6 => IPV6_DNS_ENTRIES.clone(),
        };
        self.dns_entries.lock().unwrap().extend(dns_entries);
    }

    /// Start the benchmark timer.
    fn bench_start_time(&mut self) {
        self.bench_start_time = Some(Instant::now());
    }

    /// Spawn the threads.
    fn spawn_threads(&mut self) {
        for _ in 0..self.arguments.threads {
            let dns_entries = self.dns_entries.clone();
            let result_entries = self.result_entries.clone();
            let arguments = self.arguments.clone();
            let pb = self.progress_bar.as_ref().unwrap().clone();

            self.threads.push(thread::spawn(move || loop {
                let dns_entry = {
                    let mut dns_entries = dns_entries.lock().unwrap();
                    dns_entries.pop_front()
                };

                if let Some(dns_entry) = dns_entry {
                    let mut measure_results = Vec::new();

                    for _ in 0..arguments.requests {
                        // Create a new resolver for each request to avoid caching.
                        let mut resolver_config = ResolverConfig::new();
                        resolver_config.add_name_server(NameServerConfig {
                            socket_addr: dns_entry.socker_addr,
                            protocol: arguments.protocol.into(),
                            tls_dns_name: None,
                            trust_negative_responses: false,
                            bind_addr: None,
                        });
                        let mut resolver_opts = ResolverOpts::default();
                        resolver_opts.attempts = 0;
                        resolver_opts.timeout = Duration::from_secs(arguments.timeout);
                        resolver_opts.ip_strategy = arguments.lookup_ip.into();
                        let resolver = Resolver::new(resolver_config, resolver_opts).unwrap();

                        // Measure the time it takes to resolve the domain.
                        let start_time = Instant::now();
                        let result_entry: MeasureResult =
                            match resolver.lookup_ip(arguments.domain.clone()) {
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
                                    resolved_ip: match arguments.lookup_ip {
                                        args::IpAddr::V4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                                        args::IpAddr::V6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                                    },
                                    time: TimeResult::Failed(e.to_string()),
                                },
                            };
                        measure_results.push(result_entry);
                        pb.inc(1);
                    }
                    let result_entry: ResultEntry = measure_results.into();
                    result_entries.lock().unwrap().push(result_entry);
                } else {
                    break;
                }
            }));
        }
    }

    /// Wait for all threads to finish.
    fn await_threads(&mut self) {
        for handle in self.threads.drain(..) {
            handle.join().unwrap();
        }
    }

    /// Finish and clear the progress bar.
    fn clear_progress_bar(&mut self) {
        self.progress_bar.as_ref().unwrap().finish_and_clear();
    }

    /// Sort result entries by average duration, failed entries are at the end.
    fn sort_result_entries(&self) {
        let mut result_entries = self.result_entries.lock().unwrap();
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
    }

    /// Print the result.
    fn print_result(&self) {
        let table = Table::new(&*self.result_entries.lock().unwrap()).to_string();
        println!("{}", table);
    }

    /// Print the benchmark time.
    fn print_bench_elapsed_time(&self) {
        let bench_elapsed_time = self.bench_start_time.unwrap().elapsed();
        println!("Benchmark completed in {bench_elapsed_time:?}",);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::parse();
    let mut app = DnsBenchApplication::new(arguments);
    app.run();

    Ok(())
}
