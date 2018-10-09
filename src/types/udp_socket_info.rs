use std::net::IpAddr;
use types::OsSocketInfo;

#[derive(Clone, Debug)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub pid: Option<u32>,
    pub os_specific_info: OsSocketInfo,
}
