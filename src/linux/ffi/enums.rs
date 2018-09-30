use linux::ffi::types::*;
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
pub const AF_INET: AF_TYPE = 2;
pub const AF_INET6: AF_TYPE = 10;
pub const AF_NETLINK: AF_TYPE = 16;

/*
 * From "linux/netlink.h"
 */

pub type NETLINK_TYPE = c_int;
pub const NETLINK_INET_DIAG: NETLINK_TYPE = 4;

pub type NLM_F_TYPE = c_int;
pub const NLM_F_REQUEST: NLM_F_TYPE = 1; /* It is request message. 	*/
pub const NLM_F_MULTI: NLM_F_TYPE = 2; /* Multipart message, terminated by NLMSG_DONE */
pub const NLM_F_ACK: NLM_F_TYPE = 4; /* Reply with ack, with zero or error code */
pub const NLM_F_ECHO: NLM_F_TYPE = 8; /* Echo this request 		*/
pub const NLM_F_DUMP_INTR: NLM_F_TYPE = 16; /* Dump was inconsistent due to sequence change */
pub const NLM_F_ROOT: NLM_F_TYPE = 0x100; /* specify tree root */
pub const NLM_F_MATCH: NLM_F_TYPE = 0x200; /* return all matching */
pub const NLM_F_ATOMIC: NLM_F_TYPE = 0x400; /* atomic GET */
pub const NLM_F_DUMP: NLM_F_TYPE = (NLM_F_ROOT | NLM_F_MATCH);

pub type NLMSG_TYPE = __u16;
pub const NLMSG_NOOP: NLMSG_TYPE = 0x1; /* Nothing.		*/
pub const NLMSG_ERROR: NLMSG_TYPE = 0x2; /* Error		*/
pub const NLMSG_DONE: NLMSG_TYPE = 0x3; /* End of a dump	*/
pub const NLMSG_OVERRUN: NLMSG_TYPE = 0x4; /* Data lost		*/

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

pub type GETSOCK_TYPE = c_int;
pub const TCPDIAG_GETSOCK: GETSOCK_TYPE = 18;
pub const DCCPDIAG_GETSOCK: GETSOCK_TYPE = 19;
