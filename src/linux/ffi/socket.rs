use linux::ffi::*;
use std::os::raw::*;

/*
 * From "sys/socket.h"
 */

extern "C" {
    pub fn socket(af: c_int, sock_type: c_int, protocol: c_int) -> c_int;
    pub fn sendmsg(sock_fd: c_int, msg: *const msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(sock_fd: c_int, msg: *mut msghdr, flags: c_int) -> ssize_t;
    pub fn recv(sock_fd: c_int, buf: *mut c_void, n: size_t, flags: c_int) -> ssize_t;
    pub fn shutdown(sock_fd: c_int, how: SHUT_TYPE) -> c_int;
}
