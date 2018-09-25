use types::AddressFamily;
use windows::ffi;

impl Into<ffi::ULONG> for AddressFamily {
    fn into(self) -> ffi::ULONG {
        match self {
            AddressFamily::AF_INET => ffi::AF_INET,
            AddressFamily::AF_INET6 => ffi::AF_INET6,
        }
    }
}
