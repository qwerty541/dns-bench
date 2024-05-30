use crate::args::IpAddr;
use crate::servers::DnsEntry;

use std::fs::File;
use std::io::{self, BufRead};
use std::net;
use std::path::PathBuf;

pub fn read_custom_servers_list(filepath: PathBuf, ip: IpAddr) -> io::Result<Vec<DnsEntry>> {
    let file = File::open(filepath)?;
    let reader = io::BufReader::new(file);
    let mut entries = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let Some((name, socket_addr)) = parse_line(&line, ip) else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid line"));
        };
        entries.push(DnsEntry { name, socket_addr });
    }

    Ok(entries)
}

fn parse_line(line: &str, ip: IpAddr) -> Option<(String, net::SocketAddr)> {
    let parts: Vec<&str> = line.split(';').collect();
    if parts.len() != 2 {
        return None;
    }

    let name = parts[0].to_string();
    let addr = if ip == IpAddr::V4 {
        net::SocketAddr::V4(parts[1].parse::<net::SocketAddrV4>().ok()?)
    } else {
        net::SocketAddr::V6(parts[1].parse::<net::SocketAddrV6>().ok()?)
    };

    Some((name, addr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_ipv4() {
        let line = "Google;8.8.8.8:53";
        let (name, socket_addr) = parse_line(line, IpAddr::V4).unwrap();

        assert_eq!(name, "Google");
        assert_eq!(socket_addr, "8.8.8.8:53".parse().unwrap());
    }

    #[test]
    fn test_parse_line_ipv6() {
        let line = "Google;[2001:4860:4860:0:0:0:0:8888]:53";
        let (name, socket_addr) = parse_line(line, IpAddr::V6).unwrap();

        assert_eq!(name, "Google");
        assert_eq!(
            socket_addr,
            "[2001:4860:4860:0:0:0:0:8888]:53".parse().unwrap()
        );
    }

    #[test]
    fn test_read_custom_servers_list_ipv4() {
        let filepath = PathBuf::from("./ipv4-custom-servers-example.txt");
        let entries = read_custom_servers_list(filepath, IpAddr::V4).unwrap();

        assert_eq!(entries.len(), 25);
        assert_eq!(entries[0].name, "Google");
        assert_eq!(entries[0].socket_addr, "8.8.8.8:53".parse().unwrap());
    }

    #[test]
    fn test_read_custom_servers_list_ipv6() {
        let filepath = PathBuf::from("./ipv6-custom-servers-example.txt");
        let entries = read_custom_servers_list(filepath, IpAddr::V6).unwrap();

        assert_eq!(entries.len(), 19);
        assert_eq!(entries[0].name, "Google");
        assert_eq!(
            entries[0].socket_addr,
            "[2001:4860:4860:0:0:0:0:8888]:53".parse().unwrap()
        );
    }
}
