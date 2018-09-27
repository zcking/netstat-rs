use std::os::raw::*;

/*
 * From "sys/socket.h"
 */
pub type SOCK_TYPE = c_int;
pub const SOCK_RAW: SOCK_TYPE = 3;

/*
 * From "linux/socket.h"
 */
pub type AF_TYPE = c_int;
pub const AF_NETLINK: AF_TYPE = 16;

/*
 * From "linux/netlink.h"
 */
pub type NETLINK_TYPE = c_int;
pub const NETLINK_INET_DIAG: NETLINK_TYPE = 4;
