use types::TcpSocketInfo;
use types::UdpSocketInfo;

#[derive(Copy, Clone, Debug)]
pub enum SocketInfo {
    TcpSocketInfo(TcpSocketInfo),
    UdpSocketInfo(UdpSocketInfo),
}
