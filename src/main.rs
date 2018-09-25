mod ffi;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Copy, Clone, Debug)]
pub enum AddressFamily {
    AF_INET,
    AF_INET6,
}

impl AddressFamily {
    pub fn as_ulong(&self) -> ffi::ULONG {
        match *self {
            AddressFamily::AF_INET => ffi::AF_INET,
            AddressFamily::AF_INET6 => ffi::AF_INET6,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BindingInfo {
    pub protocol: Protocol,
    pub ip: IpAddr,
    pub port: u16,
    pub pid: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum ErrorType {
    BufferInitializationError(u32),
    ErrorWithCode(u32),
}

#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub method_name: &'static str,
    pub error_type: ErrorType,
}

fn get_extended_udp_table(
    address_family: AddressFamily,
    bindings: &mut Vec<BindingInfo>,
) -> Result<(), Error> {
    unsafe {
        let af_ulong = address_family.as_ulong();
        let mut buffer_size: ffi::DWORD = 0;
        let mut err_code = ffi::GetExtendedUdpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            ffi::FALSE,
            af_ulong,
            ffi::UDP_TABLE_OWNER_PID,
            0,
        );
        let mut buffer = Vec::<u8>::new();
        let mut iterations = 0;
        while err_code == ffi::ERROR_INSUFFICIENT_BUFFER {
            buffer = Vec::<u8>::with_capacity(buffer_size as usize);
            err_code = ffi::GetExtendedUdpTable(
                buffer.as_mut_ptr() as ffi::PVOID,
                &mut buffer_size,
                ffi::FALSE,
                af_ulong,
                ffi::UDP_TABLE_OWNER_PID,
                0,
            );
            iterations += 1;
            if iterations > 100 {
                return Result::Err(Error {
                    method_name: "GetExtendedUdpTable",
                    error_type: ErrorType::BufferInitializationError(iterations),
                });
            }
        }
        if err_code == ffi::NO_ERROR {
            match address_family {
                AddressFamily::AF_INET => {
                    let table_ref = &*(buffer.as_ptr() as *const ffi::MIB_UDPTABLE_OWNER_PID);
                    let rows_count = table_ref.rows_count as usize;
                    let row_ptr = &table_ref.rows[0] as *const ffi::MIB_UDPROW_OWNER_PID;
                    for i in 0..rows_count {
                        let row = &*row_ptr.offset(i as isize);
                        bindings.push(BindingInfo {
                            protocol: Protocol::UDP,
                            ip: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                            port: u16::from_be(row.local_port as u16),
                            pid: row.owning_pid,
                        });
                    }
                }
                AddressFamily::AF_INET6 => {
                    let table_ref = &*(buffer.as_ptr() as *const ffi::MIB_UDP6TABLE_OWNER_PID);
                    let rows_count = table_ref.rows_count as usize;
                    let row_ptr = &table_ref.rows[0] as *const ffi::MIB_UDP6ROW_OWNER_PID;
                    for i in 0..rows_count {
                        let row = &*row_ptr.offset(i as isize);
                        bindings.push(BindingInfo {
                            protocol: Protocol::UDP,
                            ip: IpAddr::V6(Ipv6Addr::from(row.local_addr)),
                            port: u16::from_be(row.local_port as u16),
                            pid: row.owning_pid,
                        });
                    }
                }
            }
            return Result::Ok(());
        } else {
            return Result::Err(Error {
                method_name: "GetExtendedUdpTable",
                error_type: ErrorType::ErrorWithCode(err_code),
            });
        }
    }
}

fn main() {
    let mut bindings = Vec::<BindingInfo>::with_capacity(128);
    get_extended_udp_table(AddressFamily::AF_INET6, &mut bindings).expect("Error!!!");
    for binding in bindings {
        println!(
            "ip = {}, port = {}, pid = {}",
            binding.ip, binding.port, binding.pid
        );
    }
}
