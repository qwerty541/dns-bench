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
        let Some((name, socker_addr)) = parse_line(&line, ip) else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid line"));
        };
        entries.push(DnsEntry { name, socker_addr });
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
