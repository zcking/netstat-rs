use std::os::raw::*;

pub type __s8 = c_char;
pub type __u8 = c_uchar;
pub type __s16 = c_short;
pub type __u16 = c_ushort;
pub type __s32 = c_int;
pub type __u32 = c_uint;
pub type __s64 = c_longlong;
pub type __u64 = c_ulonglong;

pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;

pub type ssize_t = isize;

/*
 * From "sys/uio.h"
 */
pub type caddr_t = *mut c_void;
