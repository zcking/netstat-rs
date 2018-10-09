#[derive(Clone, Debug)]
pub enum OsSocketInfo {
    Windows,
    Linux(LinuxSocketInfo),
    Osx,
}

#[derive(Clone, Debug)]
pub struct LinuxSocketInfo {
    pub inode: u32,
    pub pids: Vec<u32>,
}

impl OsSocketInfo {
    pub fn expect_linux_mut(&mut self) -> &mut LinuxSocketInfo {
        match self {
            OsSocketInfo::Linux(linux_socket_info) => linux_socket_info,
            _ => panic!(),
        }
    }
}
