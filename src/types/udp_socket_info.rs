use std::net::IpAddr;

#[derive(Copy, Clone, Debug)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_scope: Option<u32>,
    pub local_port: u16,
    pub pid: u32,
}
