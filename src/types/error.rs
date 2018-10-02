#[derive(Copy, Clone, Debug)]
pub enum ErrorDetails {
    BufferInitializationError(u32),
    ErrorWithCode(u32),
}

#[derive(Copy, Clone, Debug)]
pub struct Error {
    pub method_name: &'static str,
    pub error_details: ErrorDetails,
}
