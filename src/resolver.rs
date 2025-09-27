use hickory_resolver::config::LookupIpStrategy;
use hickory_resolver::config::NameServerConfig;
use hickory_resolver::config::Protocol;
use hickory_resolver::config::ResolverConfig;
use hickory_resolver::config::ResolverOpts;
use hickory_resolver::Resolver;
use std::net::SocketAddr;
use std::time::Duration;

pub fn create_resolver(
    socket_addr: SocketAddr,
    protocol: Protocol,
    timeout_millis: u64,
    lookup_ip: LookupIpStrategy,
) -> Resolver {
    let mut resolver_config = ResolverConfig::new();
    resolver_config.add_name_server(NameServerConfig {
        socket_addr,
        protocol,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    });

    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.attempts = 0;
    resolver_opts.timeout = Duration::from_millis(timeout_millis);
    resolver_opts.ip_strategy = lookup_ip;

    Resolver::new(resolver_config, resolver_opts).unwrap()
}
