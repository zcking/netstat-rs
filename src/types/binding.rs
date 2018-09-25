use types::TcpBindingInfo;
use types::UdpBindingInfo;

#[derive(Copy, Clone, Debug)]
pub enum Binding {
    TcpBinding(TcpBindingInfo),
    UdpBinding(UdpBindingInfo),
}
