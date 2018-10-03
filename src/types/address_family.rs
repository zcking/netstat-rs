bitflags! {
    pub struct AddressFamily: u8 {
        const Ipv4 = 0b00000001;
        const Ipv6 = 0b00000010;
    }
}
