bitflags! {
    /// Set of protocols.
    pub struct Protocol: u8 {
        const TCP = 0b00000001;
        const UDP = 0b00000010;
    }
}
