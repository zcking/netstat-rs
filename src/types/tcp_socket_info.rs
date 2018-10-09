use std::net::IpAddr;
use types::{OsSocketInfo, TcpState};

#[derive(Clone, Debug)]
pub struct TcpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
    pub pid: Option<u32>,
    pub os_specific_info: OsSocketInfo,
}
