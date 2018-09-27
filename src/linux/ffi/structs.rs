use linux::ffi::types::*;
use std::os::raw::*;

/*
 * From "sys/uio.h"
 */

pub struct iovec {
    iov_base: caddr_t,
    iov_len: c_int,
}

/*
 * From "sys/socket.h"
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    msg_name: caddr_t,      /* optional address */
    msg_namelen: c_uint,    /* size of address */
    msg_iov: *const iovec,  /* scatter/gather array */
    msg_iovlen: c_uint,     /* # elements in msg_iov */
    msg_control: caddr_t,   /* ancillary data, see below */
    msg_controllen: c_uint, /* ancillary data buffer len */
    msg_flags: c_int,       /* flags on received message */
}

/*
 * From "netlink.h"
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlmsghdr {
    pub len: __u32,
    pub type_: __u16,
    pub flags: __u16,
    pub seq: __u32,
    pub pid: __u32,
}

/*
 * From "inet_diag.h"
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_sockid {
    pub sport: __be16,
    pub dport: __be16,
    pub src: [__be32; 4],
    pub dst: [__be32; 4],
    pub if_: __u32,
    pub cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_msg {
    pub family: __u8,
    pub state: __u8,
    pub timer: __u8,
    pub retrans: __u8,
    pub id: inet_diag_sockid,
    pub expires: __u32,
    pub rqueue: __u32,
    pub wqueue: __u32,
    pub uid: __u32,
    pub inode: __u32,
}
