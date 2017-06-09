// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "alloc_system"]
#![crate_type = "rlib"]
#![no_std]
#![allocator]
#![allow(dead_code, unused_variables)]
#![deny(warnings)]
#![unstable(feature = "alloc_system",
            reason = "this library is unlikely to be stabilized in its current \
                      form or name",
            issue = "27783")]
#![feature(allocator)]
#![feature(staged_api)]

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "asmjs",
              target_arch = "wasm32")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64",
              target_arch = "mips64",
              target_arch = "s390x",
              target_arch = "sparc64")))]
const MIN_ALIGN: usize = 16;

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe { imp::allocate(size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_allocate_zeroed(size: usize, align: usize) -> *mut u8 {
    unsafe { imp::allocate_zeroed(size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { imp::deallocate(ptr, old_size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    old_size: usize,
                                    size: usize,
                                    align: usize)
                                    -> *mut u8 {
    unsafe { imp::reallocate(ptr, old_size, size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            old_size: usize,
                                            size: usize,
                                            align: usize)
                                            -> usize {
    unsafe { imp::reallocate_inplace(ptr, old_size, size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, align: usize) -> usize {
    imp::usable_size(size, align)
}


mod raw;

mod kern_malloc;

mod imp {

    use kern_malloc as kern;
    use raw;
    use core::cmp;
    use core::ptr;
    use MIN_ALIGN;

    pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {

        // kern::uprintf("allocate: %d\n\0".as_ptr() as *const i8, size);

        if align <= MIN_ALIGN {
            kern::malloc(size as raw::c_size_t,
                         &mut kern::M_DEVBUF[0],
                         kern::M_WAITOK as i32) as *mut u8
        } else {
            aligned_malloc(size, align)
        }
    }

    pub unsafe fn allocate_zeroed(size: usize, align: usize) -> *mut u8 {

        // kern::uprintf("allocate_zeroed: %d\n\0".as_ptr() as *const i8, size);

        if align <= MIN_ALIGN {
            kern::malloc(size as raw::c_size_t,
                         &mut kern::M_DEVBUF[0],
                         kern::M_WAITOK as i32 | kern::M_ZERO as i32) as *mut u8
        } else {
            let ptr = aligned_malloc(size, align);
            if !ptr.is_null() {
                ptr::write_bytes(ptr, 0, size);
            }
            ptr
        }
    }


    unsafe fn aligned_malloc(size: usize, align: usize) -> *mut u8 {
        // TODO: Replace with proper call
        kern::uprintf("aligned_malloc: using normal malloc!\n\0".as_ptr() as *const i8);
        kern::malloc(size as raw::c_size_t,
                     &mut kern::M_DEVBUF[0],
                     kern::M_WAITOK as i32) as *mut u8
    }


    pub unsafe fn reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {

        // kern::uprintf("reallocate %d -> %d\n\0".as_ptr() as *const i8, old_size, size);

        if align <= MIN_ALIGN {
            kern::realloc(ptr as *mut raw::c_void,
                          size as raw::c_size_t,
                          &mut kern::M_DEVBUF[0],
                          kern::M_WAITOK as i32) as *mut u8
        } else {
            let new_ptr = allocate(size, align);
            if !new_ptr.is_null() {
                ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
                deallocate(ptr, old_size, align);
            }
            new_ptr
        }
    }

    pub unsafe fn reallocate_inplace(_ptr: *mut u8,
                                     old_size: usize,
                                     _size: usize,
                                     _align: usize)
                                     -> usize {
        kern::uprintf("reallocate in place undefined!\n\0".as_ptr() as *const i8);
        old_size
    }

    pub unsafe fn deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
        // kern::uprintf("deallocate %d\n\0".as_ptr() as *const i8, _old_size);
        kern::free(ptr as *mut raw::c_void, &mut kern::M_DEVBUF[0])
    }

    pub fn usable_size(size: usize, _align: usize) -> usize {
        size
    }
}
