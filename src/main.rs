use std::time::Instant;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::config::NameServerConfig;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::Protocol;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the IP address to test
    let socket_addr = SocketAddr::new("8.8.8.8".parse().unwrap(), 53);

    // Create a Resolver with custom configuration
    let mut resolver_config = ResolverConfig::new();
    resolver_config.add_name_server(NameServerConfig { socket_addr, protocol: Protocol::Udp, tls_dns_name: None, trust_negative_responses: false, bind_addr: None });
    let resolver_opts = ResolverOpts::default();
    let resolver = Resolver::new(resolver_config, resolver_opts)?;

    // Measure the DNS resolution time
    let start_time = Instant::now();
    let response = resolver.lookup_ip("google.com")?;

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed();

    // Print the results
    for ip in response.iter() {
        println!("Resolved IP: {:?}", ip);
    }

    println!("DNS Resolution Time: {:?}", elapsed_time);

    Ok(())
}
