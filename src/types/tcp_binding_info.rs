use std::net::IpAddr;
use types::TcpState;

#[derive(Copy, Clone, Debug)]
pub struct TcpBindingInfo {
    pub local_addr: IpAddr,
    pub local_scope: Option<u32>,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_scope: Option<u32>,
    pub remote_port: u16,
    pub state: TcpState,
    pub pid: u32,
}
