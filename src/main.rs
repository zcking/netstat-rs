extern crate libc;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

mod types;
use types::*;

// #[cfg(target_os = "windows")]
// use windows::{get_extended_tcp_table, get_extended_udp_table};

fn main() {
    unsafe {
        linux::get_socket_info();
    }
    // let mut bindings = Vec::<SocketInfo>::with_capacity(128);
    // get_extended_tcp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    // get_extended_tcp_table(AddressFamily::AF_INET6, &mut bindings).expect("Error!!!");
    // get_extended_udp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    // for binding in bindings {
    //     match binding {
    //         SocketInfo::TcpSocketInfo(binding) => println!(
    //             "{}:{} -> {}:{}, state = {:?}, pid = {}",
    //             binding.local_addr,
    //             binding.local_port,
    //             binding.remote_addr,
    //             binding.remote_port,
    //             binding.state,
    //             binding.pid
    //         ),
    //         _ => {}
    //     }
    // }
}
