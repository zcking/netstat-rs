use std::net::IpAddr;
use types::tcp_state::TcpState;

#[derive(Clone, Debug)]
pub struct SocketInfo {
    pub protocol_socket_info: ProtocolSocketInfo,
    pub pids: Vec<u32>,
    #[cfg(any(target_os = "linux", feature = "dox"))]
    #[doc(cfg(target_os = "linux"))]
    pub inode: u32,
}

#[derive(Clone, Debug)]
pub enum ProtocolSocketInfo {
    Tcp(TcpSocketInfo),
    Udp(UdpSocketInfo),
}

#[derive(Clone, Debug)]
pub struct TcpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_port: u16,
    pub state: TcpState,
}

#[derive(Clone, Debug)]
pub struct UdpSocketInfo {
    pub local_addr: IpAddr,
    pub local_port: u16,
}
