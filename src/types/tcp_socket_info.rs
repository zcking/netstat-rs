use std::net::IpAddr;
use types::TcpState;

#[derive(Clone, Debug)]
pub struct TcpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
    pub pids: Vec<u32>,
    #[cfg(target_os = "linux")]
    pub inode: u32,
}
