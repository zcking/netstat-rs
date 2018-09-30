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

pub fn netlink() -> c_int {
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
        println!("success 1");
        let mut buf = [0u8; 8192];
        iov = iovec {
            base: &mut buf[0] as *mut u8 as caddr_t,
            len: std::mem::size_of::<[u8; 8192]>() as c_int,
        };
        loop {
            let mut msg = msghdr {
                name: &mut nladdr as *mut sockaddr_nl as caddr_t,
                namelen: std::mem::size_of::<sockaddr_nl>() as c_uint,
                iov: &iov,
                iovlen: 1,
                control: std::ptr::null_mut(),
                controllen: 0,
                flags: 0,
            };
            let status = recvmsg(fd, &mut msg, 0);
            if status < 0 {
                println!("recvmsg status < 0 !");
                continue;
            }
            if status == 0 {
                return 0;
            }
            println!("success 2");
            let mut len = status as __u32;
            let mut nlh = &buf[0] as *const u8 as *const nlmsghdr;
            let nlmsghdr_size = std::mem::size_of::<nlmsghdr>() as __u32;
            while len >= nlmsghdr_size && (&*nlh).len >= nlmsghdr_size && (&*nlh).len <= len {
                let aligned_msg_body_offset = (nlmsghdr_size + 3) & !3;
                let msg_body = (nlh as *const u8).offset(aligned_msg_body_offset as isize)
                    as *const inet_diag_msg;
                if (&*nlh).seq != 123456 {
                    println!("bad nlh.seq, skipping! {}", (&*nlh).seq);
                } else {
                    tcp_show_sock(msg_body);
                }
                let aligned_msg_len = ((&*nlh).len + 3) & !3;
                println!("len = {}", len);
                len -= aligned_msg_len;
                println!("len = {}", len);
                nlh = (nlh as *const u8).offset(aligned_msg_len as isize) as *const nlmsghdr;
            }
        }
    }
}

fn tcp_show_sock(s: *const inet_diag_msg) {
    //
}

//#define NLMSG_HDRLEN	 ((int) NLMSG_ALIGN(sizeof(struct nlmsghdr)))
//#define NLMSG_LENGTH(len) ((len)+NLMSG_ALIGN(NLMSG_HDRLEN))
// #define NLMSG_DATA(nlh) ((void*)(((char*)nlh) + NLMSG_LENGTH(0)))

// #define NLMSG_ALIGNTO	4U
// #define NLMSG_ALIGN(len) ( ((len)+NLMSG_ALIGNTO-1) & ~(NLMSG_ALIGNTO-1) )

/*

#define NLMSG_NEXT(nlh,len)	 ((len) -= NLMSG_ALIGN((nlh)->nlmsg_len), \
    (struct nlmsghdr*)(((char*)(nlh)) + NLMSG_ALIGN((nlh)->nlmsg_len)))

#define NLMSG_OK(nlh,len) ((len) >= (int)sizeof(struct nlmsghdr) && \
			   (nlh)->nlmsg_len >= sizeof(struct nlmsghdr) && \
(nlh)->nlmsg_len <= (len))

		h = (struct nlmsghdr*)buf;
		while (NLMSG_OK(h, status)) {
			int err;
			struct inet_diag_msg *r = NLMSG_DATA(h);

			if (/*h->nlmsg_pid != rth->local.nl_pid ||*/
			    h->nlmsg_seq != 123456)
				goto skip_it;

			if (h->nlmsg_type == NLMSG_DONE)
				return 0;
			if (h->nlmsg_type == NLMSG_ERROR) {
				struct nlmsgerr *err = (struct nlmsgerr*)NLMSG_DATA(h);
				if (h->nlmsg_len < NLMSG_LENGTH(sizeof(struct nlmsgerr))) {
					fprintf(stderr, "ERROR truncated\n");
				} else {
					errno = -err->error;
					perror("TCPDIAG answers");
				}
				return 0;
			}
			if (!dump_fp) {
				if (!(f->families & (1<<r->idiag_family))) {
					h = NLMSG_NEXT(h, status);
					continue;
				}
				err = tcp_show_sock(h, NULL);
				if (err < 0)
					return err;
			}

skip_it:
			h = NLMSG_NEXT(h, status);
		}
		if (msg.msg_flags & MSG_TRUNC) {
			fprintf(stderr, "Message truncated\n");
			continue;
		}
		if (status) {
			fprintf(stderr, "!!!Remnant of size %d\n", status);
			exit(1);
		}
	}
	return 0;
}


static int tcp_show_sock(struct nlmsghdr *nlh, struct filter *f)
{
	struct inet_diag_msg *r = NLMSG_DATA(nlh);
	struct tcpstat s;

	s.state = r->idiag_state; 
	s.local.family = s.remote.family = r->idiag_family;
	s.lport = ntohs(r->id.idiag_sport);
	s.rport = ntohs(r->id.idiag_dport);
	if (s.local.family == AF_INET) {
		s.local.bytelen = s.remote.bytelen = 4;
	} else {
		s.local.bytelen = s.remote.bytelen = 16;
	}
	memcpy(s.local.data, r->id.idiag_src, s.local.bytelen);
	memcpy(s.remote.data, r->id.idiag_dst, s.local.bytelen);

	if (f && f->f && run_ssfilter(f->f, &s) == 0)
		return 0;

	if (netid_width)
		printf("%-*s ", netid_width, "tcp");
	if (state_width)
		printf("%-*s ", state_width, sstate_name[s.state]);

	printf("%-6d %-6d ", r->idiag_rqueue, r->idiag_wqueue);

	formatted_print(&s.local, s.lport);
	formatted_print(&s.remote, s.rport);

	if (show_options) {
		if (r->idiag_timer) {
			if (r->idiag_timer > 4)
				r->idiag_timer = 5;
			printf(" timer:(%s,%s,%d)",
			       tmr_name[r->idiag_timer],
			       print_ms_timer(r->idiag_expires),
			       r->idiag_retrans);
		}
	}
	if (show_users) {
		char ubuf[4096];
		if (find_users(r->idiag_inode, ubuf, sizeof(ubuf)) > 0)
			printf(" users:(%s)", ubuf);
	}
	if (show_details) {
		if (r->idiag_uid)
			printf(" uid:%u", (unsigned)r->idiag_uid);
		printf(" ino:%u", r->idiag_inode);
		printf(" sk:");
		if (r->id.idiag_cookie[1] != 0)
			printf("%08x", r->id.idiag_cookie[1]);
 		printf("%08x", r->id.idiag_cookie[0]);
	}
	if (show_mem || show_tcpinfo) {
		printf("\n\t");
		tcp_show_info(nlh, r);
	}

	printf("\n");

	return 0;
}

 */
