use libc::{__u32, nlmsghdr};

// #define NLMSG_OK(nlh,len) ((len) >= (int)sizeof(struct nlmsghdr) && \
// 			   (nlh)->nlmsg_len >= sizeof(struct nlmsghdr) && \
// 			   (nlh)->nlmsg_len <= (len))

macro_rules! NLMSG_OK {
    ($nlh:expr, $len:expr) => {{
        let nlmsghdr_size = std::mem::size_of::<nlmsghdr>();
        $len >= nlmsghdr_size as isize
            && (&*$nlh).nlmsg_len >= nlmsghdr_size as __u32
            && (&*$nlh).nlmsg_len <= $len as __u32
    }};
}

// #define NLMSG_ALIGNTO	4U
// #define NLMSG_ALIGN(len) ( ((len)+NLMSG_ALIGNTO-1) & ~(NLMSG_ALIGNTO-1) )

macro_rules! NLMSG_ALIGN {
    ($len:expr) => {{
        // println!("NLMSG_ALIGN({}) = {}", $len, ($len + 3) & !3);
        ($len + 3) & !3
    }};
}

// #define NLMSG_HDRLEN	 ((int) NLMSG_ALIGN(sizeof(struct nlmsghdr)))
// #define NLMSG_LENGTH(len) ((len) + NLMSG_HDRLEN)

macro_rules! NLMSG_LENGTH {
    ($len:expr) => {
        $len + NLMSG_ALIGN!(std::mem::size_of::<nlmsghdr>())
    };
}

// #define NLMSG_DATA(nlh)  ((void*)(((char*)nlh) + NLMSG_LENGTH(0)))

macro_rules! NLMSG_DATA {
    ($nlh:expr) => {
        ($nlh as *const u8).offset(NLMSG_LENGTH!(0) as isize)
    };
}

// #define NLMSG_NEXT(nlh,len)	 ((len) -= NLMSG_ALIGN((nlh)->nlmsg_len), \
// 				  (struct nlmsghdr*)(((char*)(nlh)) + NLMSG_ALIGN((nlh)->nlmsg_len)))

macro_rules! NLMSG_NEXT {
    ($nlh:expr, $len:expr) => {{
        let nlh_len = (&*$nlh).nlmsg_len;
        $len -= NLMSG_ALIGN!(nlh_len) as isize;
        ($nlh as *const u8).offset(NLMSG_ALIGN!(nlh_len) as isize) as *const nlmsghdr
    }};
}

// #define RTA_ALIGNTO	4U
// #define RTA_ALIGN(len) ( ((len)+RTA_ALIGNTO-1) & ~(RTA_ALIGNTO-1) )

macro_rules! RTA_ALIGN {
    ($len:expr) => {
        ($len + 3) & !3
    };
}

// #define RTA_OK(rta,len) ((len) >= (int)sizeof(struct rtattr) && \
// 			 (rta)->rta_len >= sizeof(struct rtattr) && \
// 			 (rta)->rta_len <= (len))

macro_rules! RTA_OK {
    ($rta:expr, $len:expr) => {
        let rtattr_size = std::mem::size_of::<rtattr>();
        $len >= rtattr_size && (&*$rta).len >= rtattr_size && (&*$rta).len <= $len
    };
}

// #define RTA_NEXT(rta,attrlen)	((attrlen) -= RTA_ALIGN((rta)->rta_len), \
// 				 (struct rtattr*)(((char*)(rta)) + RTA_ALIGN((rta)->rta_len)))

macro_rules! RTA_NEXT {
    ($rta:expr, $len:expr) => {
        let rta_len = (&*$rta).len;
        $len -= RTA_ALIGN!(rta_len);
        ($rta as *const u8).offset(RTA_ALIGN!(rta_len)) as *const rtattr
    };
}

// #define RTA_LENGTH(len)	(RTA_ALIGN(sizeof(struct rtattr)) + (len))

macro_rules! RTA_LENGTH {
    ($len:expr) => {
        $len + RTA_ALIGN!(std::mem::size_of::<rtattr>())
    };
}

// #define RTA_DATA(rta)   ((void*)(((char*)(rta)) + RTA_LENGTH(0)))

macro_rules! RTA_DATA {
    ($rta:expr) => {
        ($rta as *const u8).offset(RTA_LENGTH!(0))
    };
}

// #define INET_ADDRSTRLEN 16
// #define INET6_ADDRSTRLEN 46
