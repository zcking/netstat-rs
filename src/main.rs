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
pub struct TcpBindingInfo {
    pub local_addr: IpAddr,
    pub local_scope: Option<u32>,
    pub local_port: u16,
    pub remote_addr: IpAddr,
    pub remote_scope: Option<u32>,
    pub remote_port: u16,
    pub state: TcpState,
    pub pid: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum TcpState {
    MIB_TCP_STATE_CLOSED,
    MIB_TCP_STATE_LISTEN,
    MIB_TCP_STATE_SYN_SENT,
    MIB_TCP_STATE_SYN_RCVD,
    MIB_TCP_STATE_ESTAB,
    MIB_TCP_STATE_FIN_WAIT1,
    MIB_TCP_STATE_FIN_WAIT2,
    MIB_TCP_STATE_CLOSE_WAIT,
    MIB_TCP_STATE_CLOSING,
    MIB_TCP_STATE_LAST_ACK,
    MIB_TCP_STATE_TIME_WAIT,
    MIB_TCP_STATE_DELETE_TCB,
}

impl From<u32> for TcpState {
    fn from(tcp_state: u32) -> TcpState {
        match tcp_state {
            ffi::MIB_TCP_STATE_CLOSED => TcpState::MIB_TCP_STATE_CLOSED,
            ffi::MIB_TCP_STATE_LISTEN => TcpState::MIB_TCP_STATE_LISTEN,
            ffi::MIB_TCP_STATE_SYN_SENT => TcpState::MIB_TCP_STATE_SYN_SENT,
            ffi::MIB_TCP_STATE_SYN_RCVD => TcpState::MIB_TCP_STATE_SYN_RCVD,
            ffi::MIB_TCP_STATE_ESTAB => TcpState::MIB_TCP_STATE_ESTAB,
            ffi::MIB_TCP_STATE_FIN_WAIT1 => TcpState::MIB_TCP_STATE_FIN_WAIT1,
            ffi::MIB_TCP_STATE_FIN_WAIT2 => TcpState::MIB_TCP_STATE_FIN_WAIT2,
            ffi::MIB_TCP_STATE_CLOSE_WAIT => TcpState::MIB_TCP_STATE_CLOSE_WAIT,
            ffi::MIB_TCP_STATE_CLOSING => TcpState::MIB_TCP_STATE_CLOSING,
            ffi::MIB_TCP_STATE_LAST_ACK => TcpState::MIB_TCP_STATE_LAST_ACK,
            ffi::MIB_TCP_STATE_TIME_WAIT => TcpState::MIB_TCP_STATE_TIME_WAIT,
            ffi::MIB_TCP_STATE_DELETE_TCB => TcpState::MIB_TCP_STATE_DELETE_TCB,
            _ => panic!("Unknown TcpState!"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct UdpBindingInfo {
    pub local_addr: IpAddr,
    pub local_scope: Option<u32>,
    pub local_port: u16,
    pub pid: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum Binding {
    TcpBinding(TcpBindingInfo),
    UdpBinding(UdpBindingInfo),
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

fn get_extended_tcp_table(
    address_family: AddressFamily,
    bindings: &mut Vec<Binding>,
) -> Result<(), Error> {
    unsafe {
        let af_ulong = address_family.as_ulong();
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

fn get_extended_udp_table(
    address_family: AddressFamily,
    bindings: &mut Vec<Binding>,
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
                        bindings.push(Binding::UdpBinding(UdpBindingInfo {
                            local_addr: IpAddr::V4(Ipv4Addr::from(u32::from_be(row.local_addr))),
                            local_scope: Option::None,
                            local_port: u16::from_be(row.local_port as u16),
                            pid: row.owning_pid,
                        }));
                    }
                }
                AddressFamily::AF_INET6 => {
                    let table_ref = &*(buffer.as_ptr() as *const ffi::MIB_UDP6TABLE_OWNER_PID);
                    let rows_count = table_ref.rows_count as usize;
                    let row_ptr = &table_ref.rows[0] as *const ffi::MIB_UDP6ROW_OWNER_PID;
                    for i in 0..rows_count {
                        let row = &*row_ptr.offset(i as isize);
                        bindings.push(Binding::UdpBinding(UdpBindingInfo {
                            local_addr: IpAddr::V6(Ipv6Addr::from(row.local_addr)),
                            local_scope: Option::Some(row.local_scope_id),
                            local_port: u16::from_be(row.local_port as u16),
                            pid: row.owning_pid,
                        }));
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
    let mut bindings = Vec::<Binding>::with_capacity(128);
    get_extended_tcp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    get_extended_tcp_table(AddressFamily::AF_INET6, &mut bindings).expect("Error!!!");
    get_extended_udp_table(AddressFamily::AF_INET, &mut bindings).expect("Error!!!");
    for binding in bindings {
        match binding {
            Binding::TcpBinding(binding) => println!(
                "{}:{} -> {}:{}, state = {:?}, pid = {}",
                binding.local_addr,
                binding.local_port,
                binding.remote_addr,
                binding.remote_port,
                binding.state,
                binding.pid
            ),
            _ => {}
        }
    }
}
