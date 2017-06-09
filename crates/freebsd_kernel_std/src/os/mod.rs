// Basic C type declarations
pub mod raw;
pub mod kernel;

mod characterdevice;
mod module;
mod uio;

// Generated kernel header bindings
#[allow(dead_code, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod kernel_sys;
