mod ffi;

pub use self::ffi::*;
use std;
use std::os::raw::*;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct iov_req {
    pub nlh: nlmsghdr,
    pub r: inet_diag_req,
}

pub fn netlink() {
    unsafe {
        let fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_INET_DIAG);
        println!("fd = {}", fd);
        let mut nladdr: sockaddr_nl = Default::default();
        nladdr.family = AF_NETLINK as __kernel_sa_family_t;
        let mut req: iov_req = Default::default();
        req.nlh.len = std::mem::size_of::<iov_req>() as __u32;
        req.nlh.type_ = TCPDIAG_GETSOCK as __u16;
        req.nlh.flags = (NLM_F_ROOT | NLM_F_MATCH | NLM_F_REQUEST) as u16;
        req.nlh.pid = 0;
        req.nlh.seq = 123456;
        req.r.family = AF_INET as __u8;
        req.r.states = 0;
        req.r.ext |= 1 << (INET_DIAG_INFO - 1);
        req.r.ext |= 1 << (INET_DIAG_VEGASINFO - 1);
        req.r.ext |= 1 << (INET_DIAG_CONG - 1);
        let mut iov = iovec {
            base: &mut req as *mut iov_req as caddr_t,
            len: std::mem::size_of::<iov_req>() as c_int,
        };
        let msg = msghdr {
            name: &mut nladdr as *mut sockaddr_nl as caddr_t,
            namelen: std::mem::size_of::<sockaddr_nl>() as c_uint,
            iov: &iov,
            iovlen: 1,
            control: std::ptr::null_mut(),
            controllen: 0,
            flags: 0,
        };
        if sendmsg(fd, &msg, 0) < 0 {
            println!("fail");
        }
        println!("success");
    }
}
