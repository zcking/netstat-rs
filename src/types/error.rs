use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum ErrorDetails {
    BufferInitializationError(u32),
    ErrorWithCode(u32),
    RustError(Rc<Box<Debug>>),
}

/// General error type.
#[derive(Clone, Debug)]
pub struct Error {
    pub method_name: &'static str,
    pub error_details: ErrorDetails,
}
