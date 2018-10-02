use std;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use types::*;
use windows::ffi;

pub fn get_extended_tcp_table(
    address_family: AddressFamily,
    bindings: &mut Vec<Binding>,
) -> Result<(), Error> {
    unsafe {
        let af_ulong = address_family.into();
        let mut buffer_size: ffi::DWORD = 0;
        let mut err_code = ffi::GetExtendedTcpTable(
            std::ptr::null_mut(),
            &mut buffer_size,
            ffi::FALSE,
            af_ulong,
            ffi::TCP_TABLE_OWNER_PID_ALL,
            0,
        );
        let mut buffer = Vec::<u8>::new();
        let mut iterations = 0;
        while err_code == ffi::ERROR_INSUFFICIENT_BUFFER {
            buffer = Vec::<u8>::with_capacity(buffer_size as usize);
            err_code = ffi::GetExtendedTcpTable(
                buffer.as_mut_ptr() as ffi::PVOID,
                &mut buffer_size,
                ffi::FALSE,
                af_ulong,
                ffi::TCP_TABLE_OWNER_PID_ALL,
                0,
            );
            iterations += 1;
            if iterations > 100 {
                return Result::Err(Error {
                    method_name: "GetExtendedTcpTable",
                    error_type: ErrorType::BufferInitializationError(iterations),
                });
            }
        }
        if err_code == ffi::NO_ERROR {
            match address_family {
                AddressFamily::AF_INET => {
                    let table_ref = &*(buffer.as_ptr() as *const ffi::MIB_TCPTABLE_OWNER_PID);
                    let rows_count = table_ref.rows_count as usize;
                    let row_ptr = &table_ref.rows[0] as *const ffi::MIB_TCPROW_OWNER_PID;
                    for i in 0..rows_count {
                        let row = &*row_ptr.offset(i as isize);
                        bindings.push(Binding::TcpBinding(TcpBindingInfo {
                            local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                            local_scope: Option::None,
                            local_port: u16::from_be(row.local_port as u16),
                            remote_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.remote_addr))),
                            remote_scope: Option::None,
                            remote_port: u16::from_be(row.remote_port as u16),
                            state: TcpState::from(row.state),
                            pid: row.owning_pid,
                        }));
                    }
                }
                AddressFamily::AF_INET6 => {
                    let table_ref = &*(buffer.as_ptr() as *const ffi::MIB_TCP6TABLE_OWNER_PID);
                    let rows_count = table_ref.rows_count as usize;
                    let row_ptr = &table_ref.rows[0] as *const ffi::MIB_TCP6ROW_OWNER_PID;
                    for i in 0..rows_count {
                        let row = &*row_ptr.offset(i as isize);
                        bindings.push(Binding::TcpBinding(TcpBindingInfo {
                            local_addr: IpAddr::V6(Ipv6Addr::from(row.local_addr)),
                            local_scope: Option::Some(row.local_scope_id),
                            local_port: u16::from_be(row.local_port as u16),
                            remote_addr: IpAddr::V6(Ipv6Addr::from(row.remote_addr)),
                            remote_scope: Option::Some(row.remote_scope_id),
                            remote_port: u16::from_be(row.remote_port as u16),
                            state: TcpState::from(row.state),
                            pid: row.owning_pid,
                        }));
                    }
                }
            }
            return Result::Ok(());
        } else {
            return Result::Err(Error {
                method_name: "GetExtendedTcpTable",
                error_type: ErrorType::ErrorWithCode(err_code),
            });
        }
    }
}
