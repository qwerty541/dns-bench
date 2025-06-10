#[cfg(target_os = "linux")]
use std::fs;
use std::io;
use std::net::IpAddr;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::process::Command;
use std::str::FromStr;

/// Read DNS servers on Linux from /etc/resolv.conf
#[cfg(target_os = "linux")]
fn get_dns_linux() -> io::Result<(IpAddr, Option<IpAddr>)> {
    let content = fs::read_to_string("/etc/resolv.conf")?;
    let servers = parse_resolv_conf_content(&content);
    if servers.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No DNS servers found",
        ))
    } else {
        let second = servers.get(1).cloned();
        Ok((servers[0], second))
    }
}

#[cfg(any(test, target_os = "linux"))]
fn parse_resolv_conf_content(content: &str) -> Vec<IpAddr> {
    content
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if let Some(ip) = l.strip_prefix("nameserver ") {
                IpAddr::from_str(ip).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Read DNS servers on macOS using `scutil --dns`
#[cfg(target_os = "macos")]
fn get_dns_macos() -> io::Result<(IpAddr, Option<IpAddr>)> {
    let output = Command::new("scutil").args(&["--dns"]).output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let servers = parse_scutil_output(&text);
    if servers.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No DNS servers found",
        ))
    } else {
        let second = servers.get(1).cloned();
        Ok((servers[0], second))
    }
}

#[cfg(any(test, target_os = "macos"))]
fn parse_scutil_output(text: &str) -> Vec<IpAddr> {
    text.lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.starts_with("nameserver[") {
                l.split_whitespace()
                    .nth(2)
                    .and_then(|ip| IpAddr::from_str(ip).ok())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

/// Read DNS servers on Windows by parsing `ipconfig /all`
#[cfg(target_os = "windows")]
fn get_dns_windows() -> io::Result<(IpAddr, Option<IpAddr>)> {
    let output = Command::new("ipconfig").arg("/all").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let servers = parse_ipconfig_output(&text);
    if servers.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No DNS servers found",
        ))
    } else {
        let second = servers.get(1).cloned();
        Ok((servers[0], second))
    }
}

#[cfg(any(test, target_os = "windows"))]
fn parse_ipconfig_output(text: &str) -> Vec<IpAddr> {
    let mut servers = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("DNS Servers") || l.starts_with("DNS Server") {
            if let Some(ip_str) = l.split(':').nth(1) {
                let ip = IpAddr::from_str(ip_str.trim());
                if let Ok(ip) = ip {
                    servers.push(ip);
                }
            }
        } else if !servers.is_empty() && !l.is_empty() {
            // subsequent lines may list secondary servers
            if let Ok(ip) = IpAddr::from_str(l) {
                servers.push(ip);
            }
        }
    }
    servers
}

pub fn get_system_dns() -> io::Result<(IpAddr, Option<IpAddr>)> {
    #[cfg(target_os = "linux")]
    return get_dns_linux();
    #[cfg(target_os = "macos")]
    return get_dns_macos();
    #[cfg(target_os = "windows")]
    return get_dns_windows();
    #[allow(unreachable_code)]
    Err(io::Error::other("Unsupported platform"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_parse_resolv_conf_content() {
        let content = fs::read_to_string("examples/resolv.conf").unwrap();
        let servers = parse_resolv_conf_content(&content);
        assert_eq!(servers.len(), 2);
        assert_eq!(servers[0], IpAddr::from_str("8.8.8.8").unwrap());
        assert_eq!(servers[1], IpAddr::from_str("1.1.1.1").unwrap());
    }

    #[test]
    fn test_parse_scutil_output() {
        let text = fs::read_to_string("examples/scutil_dns.txt").unwrap();
        let servers = parse_scutil_output(&text);
        assert_eq!(servers.len(), 3);
        assert_eq!(servers[0], IpAddr::from_str("8.8.8.8").unwrap());
        assert_eq!(servers[1], IpAddr::from_str("1.1.1.1").unwrap());
        assert_eq!(servers[2], IpAddr::from_str("192.168.1.1").unwrap());
    }

    #[test]
    fn test_parse_ipconfig_output() {
        let text = fs::read_to_string("examples/ipconfig_all.txt").unwrap();
        let servers = parse_ipconfig_output(&text);
        assert_eq!(servers.len(), 2);
        assert_eq!(servers[0], IpAddr::from_str("8.8.8.8").unwrap());
        assert_eq!(servers[1], IpAddr::from_str("1.1.1.1").unwrap());
    }

    #[test]
    fn test_get_system_dns() {
        let res = get_system_dns();
        // Just ensure it runs without panic;

        assert!(res.is_ok());
        let (primary, _secondary) = res.unwrap();
        assert!(matches!(primary, IpAddr::V4(_) | IpAddr::V6(_)));
    }
}
