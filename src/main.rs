// #[cfg(target_os = "linux")]
mod linux;
mod types;
#[cfg(target_os = "windows")]
mod windows;

use types::*;
#[cfg(target_os = "windows")]
use windows::{get_extended_tcp_table, get_extended_udp_table};

fn main() {
    unsafe {
        linux::netlink();
    }
    // let mut bindings = Vec::<Binding>::with_capacity(128);
    // get_extended_tcp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    // get_extended_tcp_table(AddressFamily::AF_INET6, &mut bindings).expect("Error!!!");
    // get_extended_udp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    // for binding in bindings {
    //     match binding {
    //         Binding::TcpBinding(binding) => println!(
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
