use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::{Command, Stdio};
use std::rc::Rc;
use types::*;

pub fn get_netstat_info(
    address_family: AddressFamily,
    protocol: Protocol,
) -> Result<Vec<SocketInfo>, Error> {
    let mut results = Vec::new();
    let child = Command::new("netstat")
        .arg("-nv")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = BufReader::new(child.stdout.unwrap());
    for line in reader.lines().skip(2).filter_map(|ln| ln.ok()) {
        let parts: Vec<&str> = line
            .trim()
            .split(|c: char| c.is_whitespace() || c.is_control())
            .filter(|&s| s.len() > 0)
            .collect();
        if parts.len() < 9 {
            break;
        }
        let is_tcp = parts[0].starts_with("tcp");
        let is_udp = parts[0].starts_with("udp");
        let is_ipv4 = parts[0].ends_with("4");
        let is_ipv6 = !is_ipv4;
        let skip = is_tcp && !protocol.contains(Protocol::TCP)
            || is_udp && !protocol.contains(Protocol::UDP)
            || is_ipv4 && !address_family.contains(AddressFamily::Ipv4)
            || is_ipv6 && !address_family.contains(AddressFamily::Ipv6);
        if skip {
            continue;
        }
        let (local_addr, local_port) = split_endpoint(parts[3]);
        let (remote_addr, remote_port) = split_endpoint(parts[4]);
        let pid = match parts.len() {
            9 => parts[7],
            10 => parts[8],
            _ => panic!("Unknown netstat output format!"),
        };
        if is_tcp {
            results.push(SocketInfo {
                protocol_socket_info: ProtocolSocketInfo::Tcp(TcpSocketInfo {
                    local_addr: parse_ip(local_addr, is_ipv4)?,
                    local_port: parse_port(local_port)?,
                    remote_addr: parse_ip(remote_addr, is_ipv4)?,
                    remote_port: parse_port(remote_port)?,
                    state: TcpState::from(parts[5]),
                }),
                pids: vec![pid.parse::<u32>().map_err(wrap_error)?],
            });
        } else if is_udp {
            results.push(SocketInfo {
                protocol_socket_info: ProtocolSocketInfo::Udp(UdpSocketInfo {
                    local_addr: parse_ip(local_addr, is_ipv4)?,
                    local_port: parse_port(local_port)?,
                }),
                pids: vec![pid.parse::<u32>().map_err(wrap_error)?],
            });
        }
    }
    Result::Ok(results)
}

fn parse_ip(ip_str: &str, is_ipv4: bool) -> Result<IpAddr, Error> {
    let ip_str = remove_zone_index(ip_str);
    if ip_str == "*" {
        Result::Ok(match is_ipv4 {
            true => IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            false => IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        })
    } else {
        Result::Ok(match is_ipv4 {
            true => IpAddr::V4(ip_str.parse::<Ipv4Addr>().map_err(wrap_error)?),
            false => IpAddr::V6(ip_str.parse::<Ipv6Addr>().map_err(wrap_error)?),
        })
    }
}

fn parse_port(port_str: &str) -> Result<u16, Error> {
    match port_str {
        "*" => Result::Ok(0),
        _ => port_str.parse::<u16>().map_err(wrap_error),
    }
}

fn split_endpoint(endpoint: &str) -> (&str, &str) {
    for (i, c) in endpoint.chars().rev().enumerate() {
        if c == '.' {
            return (
                &endpoint[0..endpoint.len() - i - 1],
                &endpoint[endpoint.len() - i..],
            );
        }
    }
    (endpoint, &endpoint[0..0])
}

fn remove_zone_index(ip_str: &str) -> &str {
    ip_str.splitn(2, '%').nth(0).unwrap()
}

fn wrap_error<Err: Debug + 'static>(e: Err) -> Error {
    Error {
        method_name: "collect_netstat_info",
        error_details: ErrorDetails::RustError(Rc::new(Box::new(e))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_endpoint_default() {
        let (ip, port) = split_endpoint("192.168.48.128.123");
        assert_eq!(ip, "192.168.48.128");
        assert_eq!(port, "123");
    }
    #[test]
    fn split_endpoint_asterisk() {
        let (ip, port) = split_endpoint("*");
        assert_eq!(ip, "*");
        assert_eq!(port, "");
    }
}
