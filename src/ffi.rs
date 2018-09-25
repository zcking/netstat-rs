pub type PVOID = *mut std::os::raw::c_void;
pub type DWORD = std::os::raw::c_ulong;
pub type PDWORD = *mut DWORD;
pub type ULONG = std::os::raw::c_ulong;
pub type UCHAR = std::os::raw::c_uchar;

pub type BOOL = std::os::raw::c_int;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

pub type TCP_TABLE_CLASS = DWORD;
pub const TCP_TABLE_BASIC_LISTENER: TCP_TABLE_CLASS = 0;
pub const TCP_TABLE_BASIC_CONNECTIONS: TCP_TABLE_CLASS = 1;
pub const TCP_TABLE_BASIC_ALL: TCP_TABLE_CLASS = 2;
pub const TCP_TABLE_OWNER_PID_LISTENER: TCP_TABLE_CLASS = 3;
pub const TCP_TABLE_OWNER_PID_CONNECTIONS: TCP_TABLE_CLASS = 4;
pub const TCP_TABLE_OWNER_PID_ALL: TCP_TABLE_CLASS = 5;
pub const TCP_TABLE_OWNER_MODULE_LISTENER: TCP_TABLE_CLASS = 6;
pub const TCP_TABLE_OWNER_MODULE_CONNECTIONS: TCP_TABLE_CLASS = 7;
pub const TCP_TABLE_OWNER_MODULE_ALL: TCP_TABLE_CLASS = 8;

pub type MIB_TCP_STATE = DWORD;
pub const MIB_TCP_STATE_CLOSED: MIB_TCP_STATE = 1;
pub const MIB_TCP_STATE_LISTEN: MIB_TCP_STATE = 2;
pub const MIB_TCP_STATE_SYN_SENT: MIB_TCP_STATE = 3;
pub const MIB_TCP_STATE_SYN_RCVD: MIB_TCP_STATE = 4;
pub const MIB_TCP_STATE_ESTAB: MIB_TCP_STATE = 5;
pub const MIB_TCP_STATE_FIN_WAIT1: MIB_TCP_STATE = 6;
pub const MIB_TCP_STATE_FIN_WAIT2: MIB_TCP_STATE = 7;
pub const MIB_TCP_STATE_CLOSE_WAIT: MIB_TCP_STATE = 8;
pub const MIB_TCP_STATE_CLOSING: MIB_TCP_STATE = 9;
pub const MIB_TCP_STATE_LAST_ACK: MIB_TCP_STATE = 10;
pub const MIB_TCP_STATE_TIME_WAIT: MIB_TCP_STATE = 11;
pub const MIB_TCP_STATE_DELETE_TCB: MIB_TCP_STATE = 12;

pub type UDP_TABLE_CLASS = DWORD;
pub const UDP_TABLE_BASIC: UDP_TABLE_CLASS = 0;
pub const UDP_TABLE_OWNER_PID: UDP_TABLE_CLASS = 1;
pub const UDP_TABLE_OWNER_MODULE: UDP_TABLE_CLASS = 2;

pub type ERROR_CODE = DWORD;
pub const NO_ERROR: ERROR_CODE = 0;
pub const ERROR_INSUFFICIENT_BUFFER: ERROR_CODE = 0x7A;
pub const ERROR_INVALID_PARAMETER: ERROR_CODE = 0x57;

pub const AF_INET: ULONG = 2;
pub const AF_INET6: ULONG = 23;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPTABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_UDPROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDP6TABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_UDP6ROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDPROW_OWNER_PID {
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_UDP6ROW_OWNER_PID {
    pub local_addr: [UCHAR; 16],
    pub local_scope_id: DWORD,
    pub local_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPTABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_TCPROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCP6TABLE_OWNER_PID {
    pub rows_count: DWORD,
    pub rows: [MIB_TCP6ROW_OWNER_PID; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCPROW_OWNER_PID {
    pub state: DWORD,
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub remote_addr: DWORD,
    pub remote_port: DWORD,
    pub owning_pid: DWORD,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MIB_TCP6ROW_OWNER_PID {
    pub local_addr: [UCHAR; 16],
    pub local_scope_id: DWORD,
    pub local_port: DWORD,
    pub remote_addr: [UCHAR; 16],
    pub remote_scope_id: DWORD,
    pub remote_port: DWORD,
    pub state: DWORD,
    pub owning_pid: DWORD,
}

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetExtendedTcpTable(
        pTcpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: TCP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
    pub fn GetExtendedUdpTable(
        pUdpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
}
