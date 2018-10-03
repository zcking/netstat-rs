#[macro_use]
extern crate bitflags;
extern crate libc;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod types;
use types::*;

#[cfg(target_os = "linux")]
use linux::*;
#[cfg(target_os = "windows")]
use windows::*;

fn main() {
    let sockets_info = get_sockets_info(
        AddressFamily::Ipv4 | AddressFamily::Ipv6,
        Protocol::TCP | Protocol::UDP,
    ).unwrap();
    for socket_info in sockets_info {
        match socket_info {
            SocketInfo::TcpSocketInfo(i) => println!(
                "TCP {}:{} -> {}:{} [{}]",
                i.local_addr, i.local_port, i.remote_addr, i.remote_port, i.pid
            ),
            SocketInfo::UdpSocketInfo(i) => {
                println!("UDP {}:{} -> *:* [{}]", i.local_addr, i.local_port, i.pid)
            }
        }
    }
}
