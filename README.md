netstat
=======

<!-- 
[![Build Status](https://travis-ci.com/bitflags/bitflags.svg?branch=master)](https://travis-ci.com/bitflags/bitflags)
[![Join the chat at https://gitter.im/bitflags/Lobby](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/bitflags/Lobby?utm_source=badge&utm_medium=badge&utm_content=badge)
[![Latest version](https://img.shields.io/crates/v/bitflags.svg)](https://crates.io/crates/bitflags)
[![Documentation](https://docs.rs/bitflags/badge.svg)](https://docs.rs/bitflags)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.20+-yellow.svg)
![License](https://img.shields.io/crates/l/bitflags.svg)
-->

Cross-platform library to retrieve network sockets information.
Tries to be optimal by using low-level OS APIs instead of parsing output of command line utilities.
Provides unified interface and returns data structures which may have additional fields depending on platform.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
netstat = "0.6"
```

Example program:

```rust
extern crate netstat;

use netstat::*;

fn main() {
    let sockets_info = get_sockets_info(
        AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
        ProtocolFlags::TCP | ProtocolFlags::UDP,
    ).unwrap();
    for si in sockets_info {
        match si.protocol_socket_info {
            ProtocolSocketInfo::Tcp(pi) => println!(
                "TCP {}:{} -> {}:{} {:?} - {}",
                pi.local_addr, pi.local_port, pi.remote_addr, pi.remote_port, si.pids, pi.state
            ),
            ProtocolSocketInfo::Udp(i) => {
                println!("UDP {}:{} -> *:* {:?}", i.local_addr, i.local_port, si.pids)
            }
        }
    }
}
```

## Details

- On Windows, library uses [GetExtendedTcpTable](https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getextendedtcptable)/[GetExtendedUdpTable](https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getextendedudptable) (iphlpapi.h)
- On Linux, it uses NETLINK_INET_DIAG protocol and performs a pid lookup using `/proc/../fd/..`
- On OS X, it should ideally use sysctls, but currently just parses netstat output because I don't have a Mac and was unable to finish the implementation (it's mostly done, you can find it in unused modules under `src/integrations/osx` folder, contributions are welcome!)
