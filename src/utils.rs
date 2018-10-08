use std;
use types::*;

pub fn get_os_error_details() -> ErrorDetails {
    ErrorDetails::ErrorWithCode(
        std::io::Error::last_os_error()
            .raw_os_error()
            .map(|x| x as u32)
            .unwrap_or(0),
    )
}
