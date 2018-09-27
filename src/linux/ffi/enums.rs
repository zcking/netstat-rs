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

pub type NLM_F_TYPE = c_int;
pub const NLM_F_ROOT: NLM_F_TYPE = 0x100; /* specify tree root */
pub const NLM_F_MATCH: NLM_F_TYPE = 0x200; /* return all matching */
pub const NLM_F_ATOMIC: NLM_F_TYPE = 0x400; /* atomic GET */
pub const NLM_F_DUMP: NLM_F_TYPE = (NLM_F_ROOT | NLM_F_MATCH);

/*
 * From "linux/inet_diag.h"
 */
pub type INET_DIAG_TYPE = c_int;
pub const INET_DIAG_NONE: INET_DIAG_TYPE = 0;
pub const INET_DIAG_MEMINFO: INET_DIAG_TYPE = 1;
pub const INET_DIAG_INFO: INET_DIAG_TYPE = 2;
pub const INET_DIAG_VEGASINFO: INET_DIAG_TYPE = 3;
pub const INET_DIAG_CONG: INET_DIAG_TYPE = 4;
pub const INET_DIAG_TOS: INET_DIAG_TYPE = 5;
pub const INET_DIAG_TCLASS: INET_DIAG_TYPE = 6;
pub const INET_DIAG_SKMEMINFO: INET_DIAG_TYPE = 7;
pub const INET_DIAG_SHUTDOWN: INET_DIAG_TYPE = 8;
pub const INET_DIAG_DCTCPINFO: INET_DIAG_TYPE = 9;
pub const INET_DIAG_PROTOCOL: INET_DIAG_TYPE = 10; /* response attribute only */
pub const INET_DIAG_SKV6ONLY: INET_DIAG_TYPE = 11;
pub const INET_DIAG_LOCALS: INET_DIAG_TYPE = 12;
pub const INET_DIAG_PEERS: INET_DIAG_TYPE = 13;
pub const INET_DIAG_PAD: INET_DIAG_TYPE = 14;
pub const INET_DIAG_MARK: INET_DIAG_TYPE = 15;
pub const INET_DIAG_BBRINFO: INET_DIAG_TYPE = 16;
pub const INET_DIAG_CLASS_ID: INET_DIAG_TYPE = 17;
pub const INET_DIAG_MD5SIG: INET_DIAG_TYPE = 18;
