#[cfg(target_os = "linux")]
use std::fs;
use std::io;
use std::net::IpAddr;
#[cfg(any(test, target_os = "linux"))]
use std::net::Ipv4Addr;
use std::process::Command;
use std::str::FromStr;

#[cfg(target_os = "linux")]
fn get_gateway_addr_linux() -> io::Result<IpAddr> {
    // Primary method: read from /proc/net/route
    if let Ok(s) = fs::read_to_string("/proc/net/route") {
        if let Ok(gw) = parse_proc_net_route_content(&s) {
            return Ok(gw);
        }
    }

    // Fallback method: use `ip route show default`
    match Command::new("ip")
        .args(["route", "show", "default"])
        .output()
    {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            parse_ip_route_default_output(&text)
        }
        Ok(_) | Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "Gateway not found")),
    }
}

#[cfg(any(test, target_os = "linux"))]
fn parse_proc_net_route_content(content: &str) -> io::Result<IpAddr> {
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let cols: Vec<&str> = line.split_whitespace().collect();
        if cols.len() < 3 {
            continue;
        }
        let destination = cols[1];
        let gateway_hex = cols[2];

        if destination == "00000000" {
            if gateway_hex.len() != 8 {
                return Err(io::Error::from(io::ErrorKind::InvalidData));
            }

            // /proc/net/route stores the gateway in little-endian order.
            let mut bytes = [0u8; 4];
            for (i, idx) in (0..8).step_by(2).enumerate() {
                bytes[i] = u8::from_str_radix(&gateway_hex[idx..idx + 2], 16)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            }

            let ip = Ipv4Addr::from(u32::from_le_bytes(bytes));

            return Ok(IpAddr::from(ip));
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[cfg(any(test, target_os = "linux"))]
fn parse_ip_route_default_output(text: &str) -> io::Result<IpAddr> {
    for line in text.lines() {
        let ws: Vec<&str> = line.split_whitespace().collect();
        if ws.len() >= 3 && ws[0] == "default" {
            for (i, tok) in ws.iter().enumerate() {
                if *tok == "via" && i + 1 < ws.len() {
                    if let Ok(ip) = IpAddr::from_str(ws[i + 1]) {
                        return Ok(ip);
                    }
                }
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[cfg(target_os = "macos")]
fn get_gateway_addr_macos() -> io::Result<IpAddr> {
    if let Ok(out) = Command::new("route")
        .arg("-n")
        .arg("get")
        .arg("default")
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Ok(ip) = parse_route_get_default_output(&text) {
                return Ok(ip);
            }
        }
    }

    // fallback netstat -rn
    if let Ok(out) = Command::new("netstat").arg("-rn").output() {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            if let Ok(ip) = parse_netstat_rn_output(&text) {
                return Ok(ip);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[cfg(any(test, target_os = "macos"))]
fn parse_route_get_default_output(text: &str) -> io::Result<IpAddr> {
    for line in text.lines() {
        let l = line.trim();
        if l.starts_with("gateway:") {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(ip) = IpAddr::from_str(parts[1]) {
                    return Ok(ip);
                }
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[cfg(any(test, target_os = "macos"))]
fn parse_netstat_rn_output(text: &str) -> io::Result<IpAddr> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("default ") || trimmed.starts_with("default\t") {
            // columns typically: Destination Gateway Flags Refs Use Mtu Interface
            let cols: Vec<&str> = trimmed.split_whitespace().collect();
            if cols.len() >= 2 {
                if let Ok(ip) = IpAddr::from_str(cols[1]) {
                    return Ok(ip);
                }
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

#[cfg(target_os = "windows")]
fn get_gateway_addr_windows() -> io::Result<IpAddr> {
    let output = Command::new("route").arg("PRINT").output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    parse_route_print_output(&text)
}

#[cfg(any(test, target_os = "windows"))]
fn parse_route_print_output(text: &str) -> io::Result<IpAddr> {
    let mut in_ipv4_section = false;
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() {
            continue;
        }
        let lower = l.to_lowercase();
        if lower.contains("ipv4") {
            in_ipv4_section = true;
            continue;
        }
        if !in_ipv4_section {
            continue;
        }

        // Now parse lines with 4 or more columns; find those starting with 0.0.0.0 and second col 0.0.0.0
        let cols: Vec<&str> = l.split_whitespace().collect();
        if cols.len() >= 3 && cols[0] == "0.0.0.0" && cols[1] == "0.0.0.0" {
            // Gateway is usually cols[2]
            if let Ok(ip) = IpAddr::from_str(cols[2]) {
                return Ok(ip);
            }
        }
    }

    Err(io::Error::from(io::ErrorKind::NotFound))
}

pub fn get_gateway_addr() -> io::Result<IpAddr> {
    #[cfg(target_os = "linux")]
    return get_gateway_addr_linux();
    #[cfg(target_os = "macos")]
    return get_gateway_addr_macos();
    #[cfg(target_os = "windows")]
    return get_gateway_addr_windows();
    #[allow(unreachable_code)]
    Err(io::Error::other("Unsupported platform"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_test_asset;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn test_parse_proc_net_route_content() {
        let content = load_test_asset!("/gateway/linux_proc_net_route.txt");
        let result = parse_proc_net_route_content(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }

    #[test]
    fn test_parse_ip_route_default_output() {
        let content = load_test_asset!("/gateway/linux_ip_route_default.txt");
        let result = parse_ip_route_default_output(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }

    #[test]
    fn test_parse_route_get_default_output() {
        let content = load_test_asset!("/gateway/mac_route_get_default.txt");
        let result = parse_route_get_default_output(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }

    #[test]
    fn test_parse_netstat_rn_output() {
        let content = load_test_asset!("/gateway/mac_netstat_rn.txt");
        let result = parse_netstat_rn_output(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }

    #[test]
    fn test_parse_route_print_output() {
        let content = load_test_asset!("/gateway/win_route_print.txt");
        let result = parse_route_print_output(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }

    #[test]
    fn test_parse_route_print_output_ru() {
        let content = load_test_asset!("/gateway/win_route_print_ru.txt");
        let result = parse_route_print_output(content).unwrap();
        assert_eq!(result, IpAddr::from_str("192.168.0.1").unwrap());
    }
}
