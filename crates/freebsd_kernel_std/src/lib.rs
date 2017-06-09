// We *are* std
#![no_std]
// For `pub use ::core::{intrinsics,raw}` statements
#![feature(core_intrinsics)]
#![feature(raw)]
#![feature(untagged_unions)]
#![feature(shared)]
// #![feature(compiler_builtins_lib)]
#![feature(unique)]
#![feature(unboxed_closures)]
#![feature(macro_reexport)]
#![feature(alloc)]
#![feature(alloc_system)]
#![feature(unicode)]
#![feature(collections)]
#![feature(box_syntax)]
#![feature(rc_raw)]

extern crate alloc;
extern crate alloc_system;
#[macro_use]
#[macro_reexport(vec, format)]
extern crate collections as core_collections;

extern crate std_unicode;
extern crate spin;

#[macro_use]
pub mod macros;

// libstd-style public modules
pub mod io;
pub mod os;
pub mod error;

// Re-export modules from libcore
pub use core::any;
pub use core::cell;
pub use core::clone;
pub use core::cmp;
pub use core::convert;
pub use core::default;
pub use core::hash;
pub use core::iter;
pub use core::intrinsics;
pub use core::marker;
pub use core::mem;
pub use core::ops;
pub use core::ptr;
pub use core::raw;
pub use core::result;
pub use core::option;
pub use alloc::boxed;
pub use alloc::rc;

pub use core_collections::borrow;
pub use core_collections::fmt;
pub use core_collections::slice;
pub use core_collections::str;
pub use core_collections::string;
pub use core_collections::vec;

pub mod sync {
    pub use alloc::arc::Arc;
    pub use alloc::arc::Weak;
    pub use spin::Mutex;
    pub use spin::MutexGuard;

}

mod std {
    pub use clone;
    pub use default;
    pub use mem;
    pub use os;
    pub use option;
    pub use io;
    pub use error;
}
