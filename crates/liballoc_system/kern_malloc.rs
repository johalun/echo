//
// FreeBSD kernel malloc interface 

// Generated kernel header bindings
#![allow(warnings, unused_variables, dead_code, improper_ctypes, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use ::raw;

pub type u_char = raw::c_uchar;
pub type u_short = raw::c_ushort;
pub type u_int = raw::c_uint;
pub type u_long = raw::c_ulong;
pub const M_NOWAIT: raw::c_uint = 1;
pub const M_WAITOK: raw::c_uint = 2;
pub const M_ZERO: raw::c_uint = 256;
pub const M_NOVM: raw::c_uint = 512;
pub const M_USE_RESERVE: raw::c_uint = 1024;
pub const M_NODUMP: raw::c_uint = 2048;
pub const M_FIRSTFIT: raw::c_uint = 4096;
pub const M_BESTFIT: raw::c_uint = 8192;
pub const M_CONTIG: raw::c_uint = 16384;
pub const M_MAGIC: raw::c_uint = 877983977;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct malloc_type {
    pub ks_next: *mut malloc_type,
    pub ks_magic: u_long,
    pub ks_shortdesc: *const raw::c_char,
    pub ks_handle: *mut raw::c_void,
}
// #[test]
// fn bindgen_test_layout_malloc_type() {
//     assert_eq!(::core::mem::size_of::<malloc_type>() , 32usize);
//     assert_eq!(::core::mem::align_of::<malloc_type>() , 8usize);
// }
impl Clone for malloc_type {
    fn clone(&self) -> Self { *self }
}

// #[repr(C)]
// #[derive(Debug, Copy)]
// pub struct malloc_type_stats {
//     pub mts_memalloced: u64,
//     pub mts_memfreed: u64,
//     pub mts_numallocs: u64,
//     pub mts_numfrees: u64,
//     pub mts_size: u64,
//     pub _mts_reserved1: u64,
//     pub _mts_reserved2: u64,
//     pub _mts_reserved3: u64,
// }
// #[test]
// fn bindgen_test_layout_malloc_type_stats() {
//     assert_eq!(::core::mem::size_of::<malloc_type_stats>() , 64usize);
//     assert_eq!(::core::mem::align_of::<malloc_type_stats>() , 8usize);
// }
// impl Clone for malloc_type_stats {
//     fn clone(&self) -> Self { *self }
// }
// #[repr(C)]
// pub struct malloc_type_internal {
//     pub mti_probes: [u32; 2usize],
//     pub mti_zone: u_char,
//     pub mti_stats: [malloc_type_stats; 256usize],
// }
// #[test]
// fn bindgen_test_layout_malloc_type_internal() {
//     assert_eq!(::core::mem::size_of::<malloc_type_internal>() , 16400usize);
//     assert_eq!(::core::mem::align_of::<malloc_type_internal>() , 8usize);
// }
// #[repr(C)]
// #[derive(Debug, Copy)]
// pub struct malloc_type_stream_header {
//     pub mtsh_version: u32,
//     pub mtsh_maxcpus: u32,
//     pub mtsh_count: u32,
//     pub _mtsh_pad: u32,
// }
// #[test]
// fn bindgen_test_layout_malloc_type_stream_header() {
//     assert_eq!(::core::mem::size_of::<malloc_type_stream_header>() , 16usize);
//     assert_eq!(::core::mem::align_of::<malloc_type_stream_header>() , 4usize);
// }
// impl Clone for malloc_type_stream_header {
//     fn clone(&self) -> Self { *self }
// }
// #[repr(C)]
// #[derive(Debug, Copy)]
// pub struct malloc_type_header {
//     pub mth_name: [raw::c_char; 32usize],
// }
// #[test]
// fn bindgen_test_layout_malloc_type_header() {
//     assert_eq!(::core::mem::size_of::<malloc_type_header>() , 32usize);
//     assert_eq!(::core::mem::align_of::<malloc_type_header>() , 1usize);
// }
// impl Clone for malloc_type_header {
//     fn clone(&self) -> Self { *self }
// }
// extern "C" {
//     #[link_name = "M_CACHE"]
//     pub static mut M_CACHE: [malloc_type; 1usize];
// }
extern "C" {
    #[link_name = "M_DEVBUF"]
    pub static mut M_DEVBUF: [malloc_type; 1usize];
}
// extern "C" {
//     #[link_name = "M_TEMP"]
//     pub static mut M_TEMP: [malloc_type; 1usize];
// }
// extern "C" {
//     #[link_name = "M_IOV"]
//     pub static mut M_IOV: [malloc_type; 1usize];
// }
// extern "C" {
//     #[link_name = "malloc_mtx"]
//     pub static mut malloc_mtx: mtx;
// }
// pub type malloc_type_list_func_t =
//     ::core::option::Option<unsafe extern "C" fn(arg1: *mut malloc_type,
//                                                 arg2: *mut raw::c_void)>;
// extern "C" {
//     pub fn contigfree(addr: *mut raw::c_void, size: raw::c_ulong,
//                       type_: *mut malloc_type);
// }
// extern "C" {
//     pub fn contigmalloc(size: raw::c_ulong, type_: *mut malloc_type,
//                         flags: raw::c_int, low: vm_paddr_t, high: vm_paddr_t,
//                         alignment: raw::c_ulong, boundary: vm_paddr_t)
//      -> *mut raw::c_void;
// }
extern "C" {
    pub fn free(addr: *mut raw::c_void, type_: *mut malloc_type);
}
extern "C" {
    pub fn malloc(size: raw::c_ulong, type_: *mut malloc_type,
                  flags: raw::c_int) -> *mut raw::c_void;
}
// extern "C" {
//     pub fn malloc_init(arg1: *mut raw::c_void);
// }
// extern "C" {
//     pub fn malloc_last_fail() -> raw::c_int;
// }
// extern "C" {
//     pub fn malloc_type_allocated(type_: *mut malloc_type, size: raw::c_ulong);
// }
// extern "C" {
//     pub fn malloc_type_freed(type_: *mut malloc_type, size: raw::c_ulong);
// }
// extern "C" {
//     pub fn malloc_type_list(arg1: malloc_type_list_func_t,
//                             arg2: *mut raw::c_void);
// }
// extern "C" {
//     pub fn malloc_uninit(arg1: *mut raw::c_void);
// }
extern "C" {
    pub fn realloc(addr: *mut raw::c_void, size: raw::c_ulong,
                   type_: *mut malloc_type, flags: raw::c_int)
     -> *mut raw::c_void;
}
// extern "C" {
//     pub fn reallocf(addr: *mut raw::c_void, size: raw::c_ulong,
//                     type_: *mut malloc_type, flags: raw::c_int)
//      -> *mut raw::c_void;
// }
// extern "C" {
//     pub fn malloc_desc2type(desc: *const raw::c_char) -> *mut malloc_type;
// }

extern "C" {
    pub fn uprintf(arg1: *const raw::c_char, ...)
     -> raw::c_int;
}
