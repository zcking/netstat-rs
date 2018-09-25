pub type PVOID = *mut std::os::raw::c_void;
pub type DWORD = std::os::raw::c_ulong;
pub type PDWORD = *mut DWORD;
pub type ULONG = std::os::raw::c_ulong;
pub type UCHAR = std::os::raw::c_uchar;

pub type BOOL = std::os::raw::c_int;
pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

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
pub struct MIB_UDPROW_OWNER_PID {
    pub local_addr: DWORD,
    pub local_port: DWORD,
    pub owning_pid: DWORD,
}

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetExtendedUdpTable(
        pUdpTable: PVOID,
        pdwSize: PDWORD,
        bOrder: BOOL,
        ulAf: ULONG,
        TableClass: UDP_TABLE_CLASS,
        Reserved: ULONG,
    ) -> DWORD;
}
