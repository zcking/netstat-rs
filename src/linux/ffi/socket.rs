use linux::ffi::*;
use std::os::raw::*;

/*
 * From "sys/socket.h"
 */

extern "C" {
    pub fn socket(af: c_int, typ: c_int, protocol: c_int) -> c_int;
    pub fn sendmsg(sockfd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(sockfd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
}
