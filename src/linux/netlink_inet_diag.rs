use libc::*;
use linux::ffi::*;
use std;
use std::mem::size_of;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

const SOCKET_BUFFER_SIZE: usize = 8192;

pub fn get_socket_info() {
    unsafe {
        println!("AF_INET, IPPROTO_TCP:");
        get_socket_info_fp(AF_INET as u8, IPPROTO_TCP as u8);
        println!();
        println!("AF_INET6, IPPROTO_TCP:");
        get_socket_info_fp(AF_INET6 as u8, IPPROTO_TCP as u8);
        println!();
        println!("AF_INET, IPPROTO_UDP:");
        get_socket_info_fp(AF_INET as u8, IPPROTO_UDP as u8);
        println!();
        println!("AF_INET6, IPPROTO_UDP:");
        get_socket_info_fp(AF_INET6 as u8, IPPROTO_UDP as u8);
    }
}

unsafe fn get_socket_info_fp(family: __u8, protocol: __u8) {
    let mut recv_buf = [0u8; SOCKET_BUFFER_SIZE as usize];
    // let mut recv_buf = Vec::<u8>::with_capacity(SOCKET_BUFFER_SIZE as usize);
    let nl_sock = socket(AF_NETLINK as i32, SOCK_DGRAM, NETLINK_INET_DIAG);
    // println!("socket = {}", nl_sock);
    // println!("before send_diag_msg");
    send_diag_msg(nl_sock, family, protocol);
    // println!("after send_diag_msg");
    let mut buf_ptr = &mut recv_buf[0] as *mut u8 as *mut c_void;
    // let mut buf_ptr_ptr = &mut buf_ptr as *mut _ as *mut c_void;
    loop {
        // println!("before recv");
        let mut numbytes = recv(
            nl_sock,
            // recv_buf.as_mut_ptr() as *mut c_void,
            // &mut recv_buf[0] as *mut u8 as *mut c_void,
            buf_ptr,
            SOCKET_BUFFER_SIZE,
            0,
        );
        // println!("after recv");
        // println!("numbytes = {}", numbytes);
        let mut nlh = buf_ptr as *const u8 as *const nlmsghdr;
        while NLMSG_OK!(nlh, numbytes) {
            if (&*nlh).nlmsg_type == NLMSG_DONE as u16 {
                close(nl_sock);
                // println!("close = {}", );
                return;
            }
            if (&*nlh).nlmsg_type == NLMSG_ERROR as u16 {
                close(nl_sock);
                // println!("close = {}", close(nl_sock));
                println!("Error in netlink message\n");
                return;
            }
            let diag_msg = NLMSG_DATA!(nlh) as *const inet_diag_msg;
            let rtalen = (&*nlh).nlmsg_len - NLMSG_LENGTH!(size_of::<inet_diag_msg>()) as __u32;
            parse_diag_msg(&*diag_msg, rtalen as c_int);
            nlh = NLMSG_NEXT!(nlh, numbytes);
        }
    }
}

unsafe fn parse_ip(family: u8, bytes: &[__be32; 4]) -> IpAddr {
    match family as i32 {
        AF_INET => IpAddr::V4(Ipv4Addr::from(
            *(&bytes[0] as *const __be32 as *const [u8; 4]),
        )),
        AF_INET6 => IpAddr::V6(Ipv6Addr::from(
            *(bytes as *const [__be32; 4] as *const u8 as *const [u8; 16]),
        )),
        _ => panic!("Unknown family!"),
    }
}

unsafe fn parse_diag_msg(diag_msg: &inet_diag_msg, rtalen: c_int) {
    let src_port = u16::from_be(diag_msg.id.sport);
    let dst_port = u16::from_be(diag_msg.id.dport);
    let src_ip = parse_ip(diag_msg.family, &diag_msg.id.src);
    let dst_ip = parse_ip(diag_msg.family, &diag_msg.id.dst);
    println!("{}:{} -> {}:{}", src_ip, src_port, dst_ip, dst_port);
    // let src = ;
    // println!("sport: {}", Ipv4 diag_msg_ref);
    //
}

// void parse_diag_msg(struct inet_diag_msg *diag_msg, int rtalen) {
//     struct rtattr *attr;
//     struct tcp_info *tcpi;
//     char local_addr_buf[INET6_ADDRSTRLEN];
//     char remote_addr_buf[INET6_ADDRSTRLEN];
//     struct passwd *uid_info = NULL;
//     memset(local_addr_buf, 0, sizeof(local_addr_buf));
//     memset(remote_addr_buf, 0, sizeof(remote_addr_buf));
//     if (diag_msg->idiag_family == AF_INET) {
//         inet_ntop(AF_INET, (struct in_addr *)&(diag_msg->id.idiag_src),
//                   local_addr_buf, INET_ADDRSTRLEN);
//         inet_ntop(AF_INET, (struct in_addr *)&(diag_msg->id.idiag_dst),
//                   remote_addr_buf, INET_ADDRSTRLEN);
//     } else if (diag_msg->idiag_family == AF_INET6) {
//         inet_ntop(AF_INET6, (struct in_addr6 *)&(diag_msg->id.idiag_src),
//                   local_addr_buf, INET6_ADDRSTRLEN);
//         inet_ntop(AF_INET6, (struct in_addr6 *)&(diag_msg->id.idiag_dst),
//                   remote_addr_buf, INET6_ADDRSTRLEN);
//     }
//     fprintf(stdout, "src_ip: %s ", local_addr_buf);
//     fprintf(stdout, "dest_ip: %s ", remote_addr_buf);
//     fprintf(stdout, "sport: %u ", ntohs(diag_msg->id.idiag_sport));
//     fprintf(stdout, "dport: %u ", ntohs(diag_msg->id.idiag_dport));
//     fprintf(stdout, "inode: %u ", diag_msg->idiag_inode);
//     if (rtalen > 0) {
//         attr = (struct rtattr *)(diag_msg + 1);
//         while (RTA_OK(attr, rtalen)) {
//             if (attr->rta_type == INET_DIAG_INFO) {
//                 tcpi = (struct tcp_info *)RTA_DATA(attr);
//                 fprintf(stdout, "state: %u", tcpi->tcpi_state);
//             }
//             attr = RTA_NEXT(attr, rtalen);
//         }
//     }
//     fprintf(stdout, "\n");
// }

unsafe fn send_diag_msg(sockfd: c_int, family: __u8, protocol: __u8) -> isize {
    let mut sa: sockaddr_nl = std::mem::uninitialized();
    sa.nl_family = AF_NETLINK as sa_family_t;
    sa.nl_pid = 0;
    sa.nl_groups = 0;
    // let mut sa = sockaddr_nl {
    //     nl_family: AF_NETLINK as sa_family_t,
    //     nl_pad: 0,
    //     nl_pid: 0,
    //     nl_groups: 0,
    // };
    let mut conn_req = inet_diag_req_v2 {
        family: family,
        protocol: protocol,
        ext: 1 << (INET_DIAG_INFO - 1),
        pad: 0,
        states: TCPF_ALL,
        id: Default::default(),
    };
    let mut nlh = nlmsghdr {
        nlmsg_len: NLMSG_LENGTH!(size_of::<inet_diag_req_v2>()) as __u32,
        nlmsg_type: SOCK_DIAG_BY_FAMILY,
        nlmsg_flags: (NLM_F_DUMP | NLM_F_REQUEST) as u16,
        nlmsg_seq: 0,
        nlmsg_pid: 0,
    };
    // println!("nlh.len = {}", nlh.nlmsg_len);
    let mut iov = [
        iovec {
            iov_base: &mut nlh as *mut _ as *mut c_void,
            iov_len: size_of::<nlmsghdr>() as size_t,
        },
        iovec {
            iov_base: &mut conn_req as *mut _ as *mut c_void,
            iov_len: size_of::<inet_diag_req_v2>() as size_t,
        },
    ];
    // println!("iovec[0].len = {}", size_of::<nlmsghdr>() as c_int);
    // println!("iovec[1].len = {}", size_of::<inet_diag_req_v2>() as c_int);
    let msg = msghdr {
        msg_name: &mut sa as *mut _ as *mut c_void,
        msg_namelen: size_of::<sockaddr_nl>() as c_uint,
        msg_iov: &mut iov[0],
        msg_iovlen: 2,
        msg_control: std::ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    // println!("namelen = {}", msg.namelen);
    let retval = sendmsg(sockfd, &msg, 0);
    // println!("sendmsg retval = {}", retval);
    if retval == -1 {
        println!(
            "error = {:?}",
            std::io::Error::last_os_error().raw_os_error()
        );
    }
    retval
}
