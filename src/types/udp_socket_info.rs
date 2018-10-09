use std::net::IpAddr;

#[derive(Clone, Debug)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub pids: Vec<u32>,
    #[cfg(target_os = "linux")]
    pub inode: u32,
}
