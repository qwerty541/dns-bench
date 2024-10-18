use std::net;

#[derive(Debug, Clone)]
pub struct DnsEntry {
    pub name: String,
    pub socket_addr: net::SocketAddr,
}

macro_rules! ipv4_dns_entry {
    ($name:expr, $ip:expr, $port:expr) => {
        DnsEntry {
            name: String::from($name),
            socket_addr: net::SocketAddr::new(
                net::IpAddr::V4(net::Ipv4Addr::new($ip.0, $ip.1, $ip.2, $ip.3)),
                $port,
            ),
        }
    };
}

macro_rules! ipv6_dns_entry {
    ($name:expr, $ip:expr, $port:expr) => {
        DnsEntry {
            name: String::from($name),
            socket_addr: net::SocketAddr::new(
                net::IpAddr::V6(net::Ipv6Addr::new(
                    $ip.0, $ip.1, $ip.2, $ip.3, $ip.4, $ip.5, $ip.6, $ip.7,
                )),
                $port,
            ),
        }
    };
}

lazy_static::lazy_static! {
    pub static ref IPV4_DNS_ENTRIES: Vec<DnsEntry> = vec![
        ipv4_dns_entry!("Google", (8, 8, 8, 8), 53),
        ipv4_dns_entry!("Google", (8, 8, 4, 4), 53),
        ipv4_dns_entry!("Cloudflare", (1, 1, 1, 1), 53),
        ipv4_dns_entry!("Cloudflare", (1, 0, 0, 1), 53),
        ipv4_dns_entry!("Quad9", (9, 9, 9, 9), 53),
        ipv4_dns_entry!("Quad9", (149, 112, 112, 112), 53),
        ipv4_dns_entry!("Router", (192, 168, 0, 1), 53),
        ipv4_dns_entry!("Control D", (76, 76, 2, 0), 53),
        ipv4_dns_entry!("Control D", (76, 76, 10, 0), 53),
        ipv4_dns_entry!("OpenDNS Home", (208, 67, 222, 222), 53),
        ipv4_dns_entry!("OpenDNS Home", (208, 67, 220, 220), 53),
        ipv4_dns_entry!("CleanBrowsing", (185, 228, 168, 9), 53),
        ipv4_dns_entry!("CleanBrowsing", (185, 228, 169, 9), 53),
        ipv4_dns_entry!("AdGuard DNS", (94, 140, 14, 14), 53),
        ipv4_dns_entry!("AdGuard DNS", (94, 140, 15, 15), 53),
        ipv4_dns_entry!("Comodo Secure DNS", (8, 26, 56, 26), 53),
        ipv4_dns_entry!("Comodo Secure DNS", (8, 20, 247, 20), 53),
        ipv4_dns_entry!("Level3", (209, 244, 0, 3), 53),
        ipv4_dns_entry!("Level3", (209, 244, 0, 4), 53),
        ipv4_dns_entry!("Verisign", (64, 6, 64, 6), 53),
        ipv4_dns_entry!("Verisign", (64, 6, 65, 6), 53),
        ipv4_dns_entry!("DNS.WATCH", (84, 200, 69, 80), 53),
        ipv4_dns_entry!("DNS.WATCH", (84, 200, 70, 40), 53),
        ipv4_dns_entry!("Norton ConnectSafe", (199, 85, 126, 10), 53),
        ipv4_dns_entry!("Norton ConnectSafe", (199, 85, 127, 10), 53),
        ipv4_dns_entry!("SafeDNS", (195, 46, 39, 39), 53),
        ipv4_dns_entry!("SafeDNS", (195, 46, 39, 40), 53),
        ipv4_dns_entry!("NextDNS", (45, 90, 28, 100), 53),
        ipv4_dns_entry!("NextDNS", (45, 90, 30, 100), 53),
    ];
}

lazy_static::lazy_static! {
    pub static ref IPV6_DNS_ENTRIES: Vec<DnsEntry> = vec![
        ipv6_dns_entry!("Google", (0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888), 53),
        ipv6_dns_entry!("Google", (0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8844), 53),
        ipv6_dns_entry!("Cloudflare", (0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111), 53),
        ipv6_dns_entry!("Cloudflare", (0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1001), 53),
        ipv6_dns_entry!("Quad9", (0x2620, 0x00fe, 0, 0, 0, 0, 0, 0x00fe), 53),
        ipv6_dns_entry!("Quad9", (0x2620, 0x00fe, 0, 0, 0, 0, 0, 0x0009), 53),
        ipv6_dns_entry!("Router", (0xfe80, 0, 0, 0, 0, 0, 0, 0x0001), 53),
        ipv6_dns_entry!("Control D", (0x2606, 0x1a40, 0, 0, 0, 0, 0, 0), 53),
        ipv6_dns_entry!("Control D", (0x2606, 0x1a40, 0x0001, 0, 0, 0, 0, 0), 53),
        ipv6_dns_entry!("OpenDNS Home", (0x2620, 0x0119, 0x0035, 0, 0, 0, 0, 0x0035), 53),
        ipv6_dns_entry!("OpenDNS Home", (0x2620, 0x0119, 0x0053, 0, 0, 0, 0, 0x0053), 53),
        ipv6_dns_entry!("CleanBrowsing", (0x2a0d, 0x2a00, 0x0001, 0, 0, 0, 0, 0x0002), 53),
        ipv6_dns_entry!("CleanBrowsing", (0x2a0d, 0x2a00, 0x0002, 0, 0, 0, 0, 0x0002), 53),
        ipv6_dns_entry!("AdGuard DNS", (0x2a10, 0x50c0, 0, 0, 0, 0, 0x0ad1, 0x00ff), 53),
        ipv6_dns_entry!("AdGuard DNS", (0x2a10, 0x50c0, 0, 0, 0, 0, 0x0ad2, 0x00ff), 53),
        ipv6_dns_entry!("Verisign", (0x2620, 0x0074, 0x001b, 0, 0, 0, 0x0001, 0x0001), 53),
        ipv6_dns_entry!("Verisign", (0x2620, 0x0074, 0x001c, 0, 0, 0, 0x0002, 0x0002), 53),
        ipv6_dns_entry!("DNS.WATCH", (0x2001, 0x1608, 0x0010, 0x0025, 0, 0, 0x1c04, 0xb12f), 53),
        ipv6_dns_entry!("DNS.WATCH", (0x2001, 0x1608, 0x0010, 0x0025, 0, 0, 0x9249, 0xd69b), 53),
        ipv6_dns_entry!("NextDNS", (0x2a07, 0xa8c0, 0, 0, 0, 0, 0x006e, 0x3f39), 53),
        ipv6_dns_entry!("NextDNS", (0x2a07, 0xa8c1, 0, 0, 0, 0, 0x006e, 0x3f39), 53),
    ];
}
