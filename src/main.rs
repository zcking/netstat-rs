mod ffi;

use self::ffi::*;
use std::net::{IpAddr, Ipv4Addr};

pub enum Protocol {
    TCP,
    UDP,
}

pub struct BindingInfo {
    pub protocol: Protocol,
    pub ip: IpAddr,
    pub port: u16,
    pub pid: u32,
}

fn get_extended_udp_table() -> Result<Vec<BindingInfo>, String> {
    unsafe {
        let mut buffer_size: DWORD = 0;
        let mut err_code = GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            FALSE,
            AF_INET,
            UDP_TABLE_OWNER_PID,
            0,
        );
        let mut buffer = Vec::<u8>::new();
        while err_code == ERROR_INSUFFICIENT_BUFFER {
            buffer = Vec::<u8>::with_capacity(buffer_size as usize);
            err_code = GetExtendedUdpTable(
                buffer.as_mut_ptr() as PVOID,
                &mut buffer_size,
                FALSE,
                AF_INET,
                UDP_TABLE_OWNER_PID,
                0,
            );
        }
        if err_code == NO_ERROR {
            let table_ref = &*(buffer.as_ptr() as *const MIB_UDPTABLE_OWNER_PID);
            let rows_count = table_ref.rows_count as usize;
            let row_ptr = &table_ref.rows[0] as *const MIB_UDPROW_OWNER_PID;
            let mut result = Vec::<BindingInfo>::with_capacity(rows_count);
            for i in 0..rows_count {
                let row = &*row_ptr.offset(i as isize);
                result.push(BindingInfo {
                    protocol: Protocol::UDP,
                    ip: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                    port: u16::from_be(row.local_port as u16),
                    pid: row.owning_pid,
                });
            }
            return Result::Ok(result);
        } else {
            return Result::Err(err_code.to_string());
        }
    }
}

fn main() {
    for binding in get_extended_udp_table().unwrap() {
        println!(
            "ip = {}, port = {}, pid = {}",
            binding.ip, binding.port, binding.pid
        );
    }
}
