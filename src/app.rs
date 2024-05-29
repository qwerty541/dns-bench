use crate::args;
use crate::config;
use crate::custom;
use crate::result::MeasureResult;
use crate::result::ResultEntry;
use crate::result::TimeResult;
use crate::servers;

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
use std::process;
use std::sync;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tabled::settings::Style as TableStyle;
use tabled::Table;

/// The main application.
pub struct DnsBenchApplication {
    /// The arguments.
    arguments: args::Arguments,
    /// The configuration.
    config: config::DnsBenchConfig,
    /// The DNS entries.
    dns_entries: sync::Arc<sync::Mutex<collections::VecDeque<servers::DnsEntry>>>,
    /// The result entries.
    result_entries: sync::Arc<sync::Mutex<Vec<ResultEntry>>>,
    /// The threads.
    threads: Vec<thread::JoinHandle<()>>,
    /// The progress bar.
    progress_bar: Option<ProgressBar>,
    /// The benchmark start time.
    bench_start_time: Option<Instant>,
}

impl DnsBenchApplication {
    /// Create a new instance of the application.
    pub fn new(arguments: args::Arguments) -> Self {
        let mut config = Self::load_config();
        config.resolve_args(&arguments);

        Self {
            arguments,
            config,
            dns_entries: sync::Arc::new(sync::Mutex::new(collections::VecDeque::default())),
            result_entries: sync::Arc::new(sync::Mutex::new(Vec::new())),
            threads: Vec::new(),
            progress_bar: None,
            bench_start_time: None,
        }
    }

    /// Load the configuration from a file.
    fn load_config() -> config::DnsBenchConfig {
        match config::DnsBenchConfig::try_load_from_file() {
            config::LoadConfigResult::Loaded(c) => c,
            config::LoadConfigResult::FileDoesNotExist => config::DnsBenchConfig::default(),
            config::LoadConfigResult::Error(e) => {
                eprintln!(
                    "Failed to load config: {:?}\n\
                    Proceeding with default parameters...",
                    e
                );
                config::DnsBenchConfig::default()
            }
        }
    }

    /// Run the application.
    pub fn run(&mut self) {
        self.print_config_summary();
        self.save_config();
        self.fill_dns_entries();
        self.init_progress_bar();
        self.bench_start_time();
        self.spawn_threads();
        self.await_threads();
        self.clear_progress_bar();
        self.sort_result_entries();
        self.print_result();
        self.print_bench_elapsed_time();
    }

    /// Save the configuration to a file.
    fn save_config(&self) {
        if self.arguments.save_config {
            match self.config.write_into_file() {
                Ok(_) => println!("Configuration saved successfully."),
                Err(e) => eprintln!("Failed to save configuration: {:?}", e),
            }
        }
    }

    /// Print the configuration summary.
    fn print_config_summary(&self) {
        println!(
            "Starting DNS benchmark with the following parameters:\n\
            Domain: {}; Threads: {}; Requests: {}; Protocol: {}\n\
            Name servers: IP{}; Lookup: IP{}; Style: {}",
            self.config.domain,
            self.config.threads,
            self.config.requests,
            self.config.protocol,
            self.config.name_servers_ip,
            self.config.lookup_ip,
            self.config.style,
        );
    }

    /// Fill the DNS entries with the desired IP version.
    fn fill_dns_entries(&mut self) {
        match self.config.custom_servers_file.clone() {
            Some(filepath) => {
                let custom_entries =
                    match custom::read_custom_servers_list(filepath, self.config.name_servers_ip) {
                        Ok(entries) => entries,
                        Err(e) => {
                            eprintln!("Failed to read custom servers list: {:?}", e);
                            process::exit(1);
                        }
                    };
                println!("Using custom servers list.");
                self.dns_entries.lock().unwrap().extend(custom_entries);
            }
            None => {
                let dns_entries = match self.config.name_servers_ip {
                    args::IpAddr::V4 => servers::IPV4_DNS_ENTRIES.clone(),
                    args::IpAddr::V6 => servers::IPV6_DNS_ENTRIES.clone(),
                };
                self.dns_entries.lock().unwrap().extend(dns_entries);
            }
        }
    }

    /// Create a progress bar with the desired style.
    fn init_progress_bar(&mut self) {
        let pb = ProgressBar::new(
            (self.dns_entries.lock().unwrap().len() * self.config.requests) as u64,
        );
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

    /// Start the benchmark timer.
    fn bench_start_time(&mut self) {
        self.bench_start_time = Some(Instant::now());
    }

    /// Spawn the threads.
    fn spawn_threads(&mut self) {
        for _ in 0..self.config.threads {
            let dns_entries = self.dns_entries.clone();
            let result_entries = self.result_entries.clone();
            let config = self.config.clone();
            let pb = self.progress_bar.as_ref().unwrap().clone();

            self.threads.push(thread::spawn(move || loop {
                let dns_entry = {
                    let mut dns_entries = dns_entries.lock().unwrap();
                    dns_entries.pop_front()
                };

                if let Some(dns_entry) = dns_entry {
                    let mut measure_results = Vec::new();

                    for _ in 0..config.requests {
                        // Create a new resolver for each request to avoid caching.
                        let mut resolver_config = ResolverConfig::new();
                        resolver_config.add_name_server(NameServerConfig {
                            socket_addr: dns_entry.socket_addr,
                            protocol: config.protocol.into(),
                            tls_dns_name: None,
                            trust_negative_responses: false,
                            bind_addr: None,
                        });
                        let mut resolver_opts = ResolverOpts::default();
                        resolver_opts.attempts = 0;
                        resolver_opts.timeout = Duration::from_secs(config.timeout);
                        resolver_opts.ip_strategy = config.lookup_ip.into();
                        let resolver = Resolver::new(resolver_config, resolver_opts).unwrap();

                        // Measure the time it takes to resolve the domain.
                        let start_time = Instant::now();
                        let result_entry: MeasureResult =
                            match resolver.lookup_ip(config.domain.clone()) {
                                Ok(response) => {
                                    let elapsed_time = start_time.elapsed();
                                    MeasureResult {
                                        name: dns_entry.name.clone(),
                                        ip: dns_entry.socket_addr.ip(),
                                        resolved_ip: response.iter().next().unwrap(),
                                        time: TimeResult::Succeeded(elapsed_time),
                                    }
                                }
                                Err(e) => MeasureResult {
                                    name: dns_entry.name.clone(),
                                    ip: dns_entry.socket_addr.ip(),
                                    resolved_ip: match config.lookup_ip {
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
        let mut table = Table::new(&*self.result_entries.lock().unwrap());

        if self.config.style == args::Style::Empty {
            table.with(TableStyle::empty());
        } else if self.config.style == args::Style::Blank {
            table.with(TableStyle::blank());
        } else if self.config.style == args::Style::Ascii {
            table.with(TableStyle::ascii());
        } else if self.config.style == args::Style::Psql {
            table.with(TableStyle::psql());
        } else if self.config.style == args::Style::Markdown {
            table.with(TableStyle::markdown());
        } else if self.config.style == args::Style::Modern {
            table.with(TableStyle::modern());
        } else if self.config.style == args::Style::Sharp {
            table.with(TableStyle::sharp());
        } else if self.config.style == args::Style::Rounded {
            table.with(TableStyle::rounded());
        } else if self.config.style == args::Style::ModernRounded {
            table.with(TableStyle::modern_rounded());
        } else if self.config.style == args::Style::Extended {
            table.with(TableStyle::extended());
        } else if self.config.style == args::Style::Dots {
            table.with(TableStyle::dots());
        } else if self.config.style == args::Style::ReStructuredText {
            table.with(TableStyle::re_structured_text());
        } else if self.config.style == args::Style::AsciiRounded {
            table.with(TableStyle::ascii_rounded());
        }

        println!("{}", table);
    }

    /// Print the benchmark time.
    fn print_bench_elapsed_time(&self) {
        let bench_elapsed_time = self.bench_start_time.unwrap().elapsed();
        println!("Benchmark completed in {bench_elapsed_time:?}",);
    }
}
