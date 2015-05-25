use ffi::*;

#[repr(C)]
#[derive(PartialEq, Debug)]
pub enum LogLevel {
    Panic = 0,
    Error,
    Warning,
    Info,
    Debug,
    Last,
}
