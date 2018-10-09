use integrations::linux::netlink_inet_diag::*;
use integrations::linux::procfs::*;
use libc::*;
use types::*;

pub fn get_sockets_info(
    address_family: AddressFamily,
    protocol: Protocol,
) -> Result<Vec<SocketInfo>, Error> {
    unsafe {
        let mut results = Vec::new();
        let ipv4 = address_family.contains(AddressFamily::Ipv4);
        let ipv6 = address_family.contains(AddressFamily::Ipv6);
        let tcp = protocol.contains(Protocol::TCP);
        let udp = protocol.contains(Protocol::UDP);
        if ipv4 {
            if tcp {
                collect_sockets_info(AF_INET as u8, IPPROTO_TCP as u8, &mut results)?;
            }
            if udp {
                collect_sockets_info(AF_INET as u8, IPPROTO_UDP as u8, &mut results)?;
            }
        }
        if ipv6 {
            if tcp {
                collect_sockets_info(AF_INET6 as u8, IPPROTO_TCP as u8, &mut results)?;
            }
            if udp {
                collect_sockets_info(AF_INET6 as u8, IPPROTO_UDP as u8, &mut results)?;
            }
        }
        attach_pids(&mut results);
        Result::Ok(results)
    }
}

fn attach_pids(sockets_info: &mut Vec<SocketInfo>) {
    let mut pids_by_inode = build_hash_of_pids_by_inode();
    for socket_info in sockets_info.iter_mut() {
        match socket_info {
            SocketInfo::TcpSocketInfo(tcpi) => {
                tcpi.pids = pids_by_inode
                    .remove(&tcpi.inode)
                    .unwrap_or_default()
                    .iter()
                    .map(|x| *x)
                    .collect();
            }
            SocketInfo::UdpSocketInfo(udpi) => {
                udpi.pids = pids_by_inode
                    .remove(&udpi.inode)
                    .unwrap_or_default()
                    .iter()
                    .map(|x| *x)
                    .collect();
            }
        }
    }
}
