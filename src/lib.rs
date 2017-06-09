#![feature(custom_attribute, lang_items)]
#![no_std]
#![allow(unused_variables)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(box_syntax)]

#[macro_use]
extern crate std;
#[macro_use]
extern crate lazy_static;
extern crate spin;
// extern crate rlibc;

mod lang;
mod module;
mod interface;

// Will be optimized away in release build if not re-exported here.
pub use interface::module_event;

// Soft float functions that are missing.
// We don't use floats in the kernel anyway so just keep
// empty impl for now.
// TODO: Patch core to remove float completely?
#[no_mangle]
pub extern "C" fn __eqsf2() {}
#[no_mangle]
pub extern "C" fn __eqdf2() {}
#[no_mangle]
pub extern "C" fn __floatundisf() {}
#[no_mangle]
pub extern "C" fn __floatundidf() {}
#[no_mangle]
pub extern "C" fn __mulsf3() {}
#[no_mangle]
pub extern "C" fn __muldf3() {}
#[no_mangle]
pub extern "C" fn __divsf3() {}
#[no_mangle]
pub extern "C" fn __divdf3() {}

// 128 bit integer stuff (we don't use it so stub ok for now...)
#[no_mangle]
pub extern "C" fn __umodti3() {}
#[no_mangle]
pub extern "C" fn __muloti4() {}
#[no_mangle]
pub extern "C" fn __udivti3() {}
