use std::net::IpAddr;
use types::OsSocketInfo;

#[derive(Copy, Clone, Debug)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub pid: u32,
    pub os_specific_info: OsSocketInfo,
}
