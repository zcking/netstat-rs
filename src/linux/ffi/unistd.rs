use std::os::raw::*;

extern "C" {
    pub fn close(fd: c_int) -> c_int;
}
