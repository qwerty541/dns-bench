use crate::args::Format;
use crate::args::IpAddr as ArgIpAddr;
use crate::args::Style;
use crate::cli;
use crate::config;
use crate::custom;
use crate::gateway::get_gateway_addr;
use crate::resolver::create_resolver;
use crate::result::convert_result_entries_to_csv_string;
use crate::result::convert_result_entries_to_xml_string;
use crate::result::CsvResultEntry;
use crate::result::JsonResultEntry;
use crate::result::MeasureResult;
use crate::result::RawResultEntry;
use crate::result::TabledResultEntry;
use crate::result::TimeResult;
use crate::result::XmlResultEntry;
use crate::servers;
use crate::system::get_system_dns;

use indicatif::MultiProgress;
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
use tabled::settings as tabled_settings;
use tabled::Table;

const POISONED_MUTEX_ERR: &str = "Poisoned mutex error";

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
                    if gateway_ip.is_ipv4() && self.config.name_servers_ip == ArgIpAddr::V4
                        || gateway_ip.is_ipv6() && self.config.name_servers_ip == ArgIpAddr::V6
                    {
                        let already_present = entries
                            .iter()
                            .map(|e| e.socket_addr.ip())
                            .collect::<std::collections::HashSet<_>>();
                        if !already_present.contains(&gateway_ip) {
                            let socket_addr = std::net::SocketAddr::new(gateway_ip, 53);
                            let resolver = create_resolver(
                                socket_addr,
                                self.config.protocol.into(),
                                200,
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
                .collect::<std::collections::HashSet<_>>();
            for sys_ip in system_ips {
                if !already_present.contains(sys_ip)
                    && (sys_ip.is_ipv4() && self.config.name_servers_ip == ArgIpAddr::V4
                        || sys_ip.is_ipv6() && self.config.name_servers_ip == ArgIpAddr::V6)
                {
                    // Add as "System DNS"
                    let name = "System DNS".to_string();
                    // Use default port 53
                    let socket_addr = std::net::SocketAddr::new(*sys_ip, 53);
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
                    progress_bar.enable_steady_tick(Duration::from_millis(50));
                    progress_bar.set_message(format!(
                        "{} ({})",
                        dns_entry.name,
                        dns_entry.socket_addr.ip()
                    ));

                    let mut measure_results = Vec::new();

                    for _ in 0..config.requests {
                        // Create a new resolver for each request to avoid caching.
                        let resolver = create_resolver(
                            dns_entry.socket_addr,
                            config.protocol.into(),
                            config.timeout * 1000_u64,
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
        match self.config.format {
            Format::HumanReadable => self.print_result_human_readable(),
            Format::Json => self.print_result_json(),
            Format::Xml => self.print_result_xml(),
            Format::Csv => self.print_result_csv(),
        }
    }

    /// Print the result in human-readable format.
    fn print_result_human_readable(&self) {
        let result_entries = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        let system_ips = self.system_dns_ips.clone().unwrap_or_default();
        let tabled_result_entries = result_entries
            .iter()
            .cloned()
            .map(|entry| {
                let mut tre = TabledResultEntry::from(entry);
                if system_ips.contains(&tre.ip) {
                    tre.name = format!("> {}", tre.name);
                }
                tre
            })
            .collect::<Vec<TabledResultEntry>>();
        let mut table = Table::new(tabled_result_entries.clone());

        match self.config.style {
            Style::Empty => table.with(tabled_settings::Style::empty()),
            Style::Blank => table.with(tabled_settings::Style::blank()),
            Style::Ascii => table.with(tabled_settings::Style::ascii()),
            Style::Psql => table.with(tabled_settings::Style::psql()),
            Style::Markdown => table.with(tabled_settings::Style::markdown()),
            Style::Modern => table.with(tabled_settings::Style::modern()),
            Style::Sharp => table.with(tabled_settings::Style::sharp()),
            Style::Rounded => table.with(tabled_settings::Style::rounded()),
            Style::ModernRounded => table.with(tabled_settings::Style::modern_rounded()),
            Style::Extended => table.with(tabled_settings::Style::extended()),
            Style::Dots => table.with(tabled_settings::Style::dots()),
            Style::ReStructuredText => table.with(tabled_settings::Style::re_structured_text()),
            Style::AsciiRounded => table.with(tabled_settings::Style::ascii_rounded()),
        };

        for (i, entry) in tabled_result_entries.iter().enumerate() {
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 3))
                    .with(entry.successful_requests_color.clone()),
            );
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 4))
                    .with(entry.first_duration_color.clone()),
            );
            table.with(
                tabled_settings::Modify::new(tabled_settings::object::Cell::new(i + 1, 5))
                    .with(entry.average_duration_color.clone()),
            );
        }

        println!("{table}");
    }

    /// Print the result in JSON format.
    fn print_result_json(&self) {
        let result_entries = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        let json_result_entries = result_entries
            .iter()
            .cloned()
            .map(JsonResultEntry::from)
            .collect::<Vec<JsonResultEntry>>();
        match serde_json::to_string_pretty(&json_result_entries) {
            Ok(json) => println!("{json}"),
            Err(e) => eprintln!("Failed to serialize results to JSON: {e:?}"),
        }
    }

    /// Print the result in XML format.
    fn print_result_xml(&self) {
        let result_entries = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        let xml_result_entries = result_entries
            .iter()
            .cloned()
            .map(XmlResultEntry::from)
            .collect::<Vec<XmlResultEntry>>();
        match convert_result_entries_to_xml_string(xml_result_entries) {
            Ok(xml) => println!("{xml}"),
            Err(e) => eprintln!("Failed to serialize results to XML: {e:?}"),
        }
    }

    /// Print the result in CSV format.
    fn print_result_csv(&self) {
        let result_entries = self.result_entries.lock().expect(POISONED_MUTEX_ERR);
        let csv_result_entries = result_entries
            .iter()
            .cloned()
            .map(CsvResultEntry::from)
            .collect::<Vec<CsvResultEntry>>();
        match convert_result_entries_to_csv_string(csv_result_entries) {
            Ok(csv) => println!("{csv}"),
            Err(e) => eprintln!("Failed to serialize results to CSV: {e:?}"),
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
