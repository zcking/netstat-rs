#![feature(doc_cfg)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate libc;

mod integrations;
mod types;
mod utils;

pub use integrations::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sockets_info_result_is_ok() {
        let af_flags_combs = (0..AddressFamily::all().bits() + 1)
            .filter_map(|x| AddressFamily::from_bits(x))
            .collect::<Vec<AddressFamily>>();
        let proto_flags_combs = (0..Protocol::all().bits() + 1)
            .filter_map(|x| Protocol::from_bits(x))
            .collect::<Vec<Protocol>>();
        for af_flags in af_flags_combs.iter() {
            for proto_flags in proto_flags_combs.iter() {
                assert!(get_sockets_info(*af_flags, *proto_flags).is_ok());
            }
        }
    }

    #[test]
    fn get_sockets_info_result_is_empty_for_empty_flags() {
        let sockets_info = get_sockets_info(AddressFamily::empty(), Protocol::empty()).unwrap();
        assert!(sockets_info.len() == 0);
    }
}