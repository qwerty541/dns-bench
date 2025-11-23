use crate::args::Format;
use crate::args::IpAddr as ArgIpAddr;
use crate::cli;
use crate::config;
use crate::custom;
use crate::gateway::get_gateway_addr;
use crate::output::get_output_formatter;
use crate::output::OutputFormatterContext;
use crate::resolver::create_resolver;
use crate::result::MeasureResult;
use crate::result::RawResultEntry;
use crate::result::TimeResult;
use crate::servers;
use crate::system::get_system_dns;

use indicatif::MultiProgress;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::collections;
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::process;
use std::sync;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const PROGRESS_BAR_TICK_INTERVAL_MILLIS: u64 = 50;
const GATEWAY_RESPONSIVENESS_TEST_TIMEOUT_MILLIS: u64 = 200;
const POISONED_MUTEX_ERR: &str = "Poisoned mutex error";

const REDUCE_TIMEOUT_AFTER_CONSECUTIVE_FAILURES: u32 = 10;
const REDUCED_TIMEOUT_MS: u64 = 500;
const ABORT_AFTER_CONSECUTIVE_FAILURES: u32 = 15;
const MINIMAL_TIMEOUT_MS: u64 = 100;

/// The main application.
pub struct BenchmarkRunner {
    /// The arguments.
    arguments: cli::DefaultArgs,
    /// The configuration.
    config: config::DnsBenchConfig,
    /// The DNS entries.
    dns_entries: sync::Arc<sync::Mutex<collections::VecDeque<servers::DnsEntry>>>,
    /// The result entries.
    result_entries: sync::Arc<sync::Mutex<Vec<RawResultEntry>>>,
    /// The threads.
    threads: Vec<thread::JoinHandle<()>>,
    /// The progress bar.
    multi_progress: Option<MultiProgress>,
    /// The benchmark start time.
    bench_start_time: Option<Instant>,
    /// The set of system DNS server IPs (for marking in table).
    system_dns_ips: Option<Vec<IpAddr>>,
}

impl BenchmarkRunner {
    /// Create a new instance of the application.
    pub fn new(arguments: cli::DefaultArgs) -> Self {
        let mut config = config::DnsBenchConfig::try_load_from_file().unwrap_or_default();
        config.resolve_args(&arguments.args);

        // Try to get system DNS servers here and store their IPs for later marking.
        let system_dns_ips = if !config.skip_system_servers {
            match get_system_dns() {
                Ok((primary, secondary)) => {
                    let mut ips = vec![primary];
                    if let Some(sec) = secondary {
                        if sec != primary {
                            ips.push(sec);
                        }
                    }
                    Some(ips)
                }
                Err(e) => {
                    eprintln!(
                        "Failed to retrieve system DNS servers: {e}.\n\
                        Proceeding with built-in or custom list only..."
                    );
                    None
                }
            }
        } else {
            None
        };

        Self {
            arguments,
            config,
            dns_entries: sync::Arc::new(sync::Mutex::new(collections::VecDeque::default())),
            result_entries: sync::Arc::new(sync::Mutex::new(Vec::new())),
            threads: Vec::new(),
            multi_progress: None,
            bench_start_time: None,
            system_dns_ips,
        }
    }

    /// Run the application.
    pub fn run(&mut self) {
        self.print_config_summary();
        self.save_config();
        self.fill_dns_entries();
        self.init_multi_progress();
        self.bench_start_time();
        self.spawn_threads();
        self.await_threads();
        self.sort_result_entries();
        self.print_result();
        self.print_bench_elapsed_time();
    }

    /// Save the configuration to a file.
    fn save_config(&self) {
        if self.arguments.save_config {
            match self.config.write_into_file() {
                Ok(_) => println!("Configuration saved successfully."),
                Err(e) => eprintln!("Failed to save configuration: {e:?}"),
            }
        }
    }

    /// Print the configuration summary.
    fn print_config_summary(&self) {
        if self.config.format == Format::HumanReadable {
            println!(
                "Starting DNS benchmark with the following parameters:\n\
                Domain: {}; Threads: {}; Requests: {}; Timeout: {}\n\
                Protocol: {}; Name servers: IP{}; Lookup: IP{}; Style: {}",
                self.config.domain,
                self.config.threads,
                self.config.requests,
                self.config.timeout,
                self.config.protocol,
                self.config.name_servers_ip,
                self.config.lookup_ip,
                self.config.style,
            );
        }
    }

    /// Fill the DNS entries with the desired IP version.
    fn fill_dns_entries(&mut self) {
        // 1. Get the base list (custom or built-in)
        let mut entries = match self.config.custom_servers_file.clone() {
            Some(filepath) => {
                let custom_entries =
                    match custom::read_custom_servers_list(filepath, self.config.name_servers_ip) {
                        Ok(entries) => entries,
                        Err(e) => {
                            eprintln!("Failed to read custom servers list: {e:?}");
                            process::exit(1);
                        }
                    };
                println!("Using custom servers list.");
                custom_entries
            }
            None => match self.config.name_servers_ip {
                ArgIpAddr::V4 => servers::IPV4_DNS_ENTRIES.clone(),
                ArgIpAddr::V6 => servers::IPV6_DNS_ENTRIES.clone(),
            },
        };

        // 2. Try to get gateway DNS and add if not already present
        if !self.config.skip_gateway_detection {
            match get_gateway_addr() {
                Ok(gateway_ip) => {
                    let is_ip_version_matching = (gateway_ip.is_ipv4()
                        && self.config.name_servers_ip == ArgIpAddr::V4)
                        || (gateway_ip.is_ipv6() && self.config.name_servers_ip == ArgIpAddr::V6);

                    if is_ip_version_matching {
                        let already_present = entries
                            .iter()
                            .map(|e| e.socket_addr.ip())
                            .collect::<collections::HashSet<_>>();
                        if !already_present.contains(&gateway_ip) {
                            let socket_addr = SocketAddr::new(gateway_ip, 53);
                            let resolver = create_resolver(
                                socket_addr,
                                self.config.protocol.into(),
                                GATEWAY_RESPONSIVENESS_TEST_TIMEOUT_MILLIS,
                                self.config.lookup_ip.into(),
                            );
                            // Test if the gateway DNS is responsive by making a simple query
                            match resolver.lookup_ip("google.com") {
                                Ok(_) => {
                                    let name = "Router (Gateway) DNS".to_string();
                                    entries.push(servers::DnsEntry { name, socket_addr });
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Gateway DNS at {socket_addr} is not responsive: {e}"
                                    );
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to detect gateway IP address: {e}");
                }
            }
        }

        // 3. Add system DNS servers if available and not already present
        if let Some(system_ips) = &self.system_dns_ips {
            let mut already_present = entries
                .iter()
                .map(|e| e.socket_addr.ip())
                .collect::<collections::HashSet<_>>();
            for sys_ip in system_ips {
                let is_ip_version_matching = (sys_ip.is_ipv4()
                    && self.config.name_servers_ip == ArgIpAddr::V4)
                    || (sys_ip.is_ipv6() && self.config.name_servers_ip == ArgIpAddr::V6);
                let is_already_present = already_present.contains(sys_ip);

                if !is_already_present && is_ip_version_matching {
                    // Add as "System DNS"
                    let name = "System DNS".to_string();
                    // Use default port 53
                    let socket_addr = SocketAddr::new(*sys_ip, 53);
                    entries.push(servers::DnsEntry { name, socket_addr });
                    already_present.insert(*sys_ip);
                }
            }
        }

        // 4. Store entries
        self.dns_entries
            .lock()
            .expect(POISONED_MUTEX_ERR)
            .extend(entries);
    }

    /// Create a multi progress.
    fn init_multi_progress(&mut self) {
        let multi_progress = MultiProgress::new();
        self.multi_progress = Some(multi_progress);
    }

    /// Initialize a progress bar.
    fn init_progress_bar(requests_count: u64) -> ProgressBar {
        let progress_bar = ProgressBar::new(requests_count);

        let style = ProgressStyle::default_bar()
            .template("{spinner:.bold.cyan} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-");
        progress_bar.set_style(style);

        progress_bar
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
            let multi_progress = self.multi_progress.as_ref().unwrap().clone();

            self.threads.push(thread::spawn(move || loop {
                let dns_entry = {
                    let mut dns_entries = dns_entries.lock().expect(POISONED_MUTEX_ERR);
                    dns_entries.pop_front()
                };

                if let Some(dns_entry) = dns_entry {
                    let progress_bar =
                        multi_progress.add(Self::init_progress_bar(config.requests as u64));
                    progress_bar.enable_steady_tick(Duration::from_millis(
                        PROGRESS_BAR_TICK_INTERVAL_MILLIS,
                    ));
                    progress_bar.set_message(format!(
                        "{} ({})",
                        dns_entry.name,
                        dns_entry.socket_addr.ip()
                    ));

                    let mut measure_results = Vec::new();

                    // Adaptive timeout state
                    let base_timeout_ms = config.timeout * 1000_u64;
                    let mut current_timeout_ms = base_timeout_ms;
                    let mut consecutive_timeout_failures: u32 = 0;

                    for _ in 0..config.requests {
                        // Create a new resolver for each request with current adaptive timeout.
                        let resolver = create_resolver(
                            dns_entry.socket_addr,
                            config.protocol.into(),
                            current_timeout_ms,
                            config.lookup_ip.into(),
                        );

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
                                        ArgIpAddr::V4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                                        ArgIpAddr::V6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
                                    },
                                    time: TimeResult::Failed(e.to_string()),
                                },
                            };

                        // Adaptive logic: inspect the result and potentially adjust timeout / abort.
                        match &result_entry.time {
                            TimeResult::Succeeded(_) => {
                                // Reset failure streak on any success.
                                consecutive_timeout_failures = 0;
                            }
                            err @ TimeResult::Failed(_) => {
                                if err.is_timeout() {
                                    consecutive_timeout_failures += 1;

                                    // Reduce timeout after 10 consecutive timeouts (if not already reduced).
                                    if consecutive_timeout_failures
                                        >= REDUCE_TIMEOUT_AFTER_CONSECUTIVE_FAILURES
                                        && current_timeout_ms > REDUCED_TIMEOUT_MS
                                    {
                                        current_timeout_ms = REDUCED_TIMEOUT_MS;
                                    }

                                    // Abort after 15 consecutive timeouts.
                                    if consecutive_timeout_failures
                                        >= ABORT_AFTER_CONSECUTIVE_FAILURES
                                    {
                                        // Minimal timeout to speed up remaining requests.
                                        current_timeout_ms = MINIMAL_TIMEOUT_MS;
                                    }
                                } else {
                                    // Non-timeout failure does not advance timeout streak (and resets it to avoid accidental decay).
                                    consecutive_timeout_failures = 0;
                                }
                            }
                        }

                        measure_results.push(result_entry);
                        progress_bar.inc(1);
                    }

                    let result_entry: RawResultEntry = measure_results.into();
                    result_entries
                        .lock()
                        .expect(POISONED_MUTEX_ERR)
                        .push(result_entry);

                    progress_bar.finish_and_clear();
                    multi_progress.remove(&progress_bar);
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

    /// Sort result entries by average duration, failed entries are at the end.
    fn sort_result_entries(&self) {
        let mut result_entries = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        result_entries.sort_by(|a, b| {
            let a = match a.avg_duration {
                TimeResult::Succeeded(duration) => duration,
                TimeResult::Failed(_) => Duration::new(u64::MAX, 0),
            };
            let b = match b.avg_duration {
                TimeResult::Succeeded(duration) => duration,
                TimeResult::Failed(_) => Duration::new(u64::MAX, 0),
            };
            a.cmp(&b)
        });
    }

    /// Print the result.
    fn print_result(&self) {
        let results = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        let formatter = get_output_formatter(&self.config.format);
        let ctx = OutputFormatterContext {
            system_dns_ips: self.system_dns_ips.clone(),
            config: self.config.clone(),
        };
        match formatter.write(&results, ctx, &mut io::stdout()) {
            Ok(()) => {}
            Err(e) => eprintln!("Error writing output: {}", e),
        }
    }

    /// Print the benchmark time.
    fn print_bench_elapsed_time(&self) {
        if self.config.format == Format::HumanReadable {
            let bench_elapsed_time = self.bench_start_time.unwrap().elapsed();
            println!("Benchmark completed in {bench_elapsed_time:?}",);
        }
    }
}
