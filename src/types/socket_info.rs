use types::TcpSocketInfo;
use types::UdpSocketInfo;

#[derive(Clone, Debug)]
pub enum SocketInfo {
    TcpSocketInfo(TcpSocketInfo),
    UdpSocketInfo(UdpSocketInfo),
}
