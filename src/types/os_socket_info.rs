#[derive(Copy, Clone, Debug)]
pub enum OsSocketInfo {
    Windows,
    Linux(LinuxSocketInfo),
    Osx,
}

#[derive(Copy, Clone, Debug)]
pub struct LinuxSocketInfo {
    pub inode: u32,
}
