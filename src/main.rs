#[macro_use]
extern crate bitflags;
extern crate libc;

mod integrations;
mod types;
mod utils;

use integrations::*;
use types::*;

fn main() {
    let sockets_info = get_sockets_info(
        AddressFamily::Ipv4 | AddressFamily::Ipv6,
        Protocol::TCP | Protocol::UDP,
    ).unwrap();
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(pi) => println!(
                "TCP {}:{} -> {}:{} ({}) [{:?}]",
                pi.local_addr, pi.local_port, pi.remote_addr, pi.remote_port, pi.state, si.pids
            ),
            ProtocolSocketInfo::Udp(i) => println!(
                "UDP {}:{} -> *:* [{:?}]",
                i.local_addr, i.local_port, si.pids
            ),
        }
    }
}
