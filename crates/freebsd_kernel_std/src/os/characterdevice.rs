
use spin::{Mutex, MutexGuard};
use os::raw;
use core::ptr;
use core::mem;
use core::cmp;
use core::ops;
use core::marker::Send;
use core::marker::Sync;
use alloc::boxed::Box;
use alloc::arc::Arc;
use core::cell::UnsafeCell;
use core::cell::RefCell;
use core::ptr::Shared;
use string::String;
use fmt;
use core::sync::atomic::AtomicUsize;
use core::ops::{Deref, DerefMut};
use io;

use super::kernel::*;

use super::kernel_sys as ksys;



pub trait CharacterDevice {
    fn open(&mut self) {}
    fn close(&mut self) {}
    fn read(&mut self, mut uio: UioWriter) {}
    fn write(&mut self, mut uio: UioReader) {}
}



pub struct CDev<T>
    where T: CharacterDevice
{
    cdev: ptr::Unique<ksys::cdev>,
    delegate: SharedModule<T>,
}


impl<T> CDev<T>
    where T: CharacterDevice
{
    pub fn new_with_delegate(name: &'static str, delegate: SharedModule<T>) -> Option<Box<Self>> {

        let mut cdevsw_raw: *mut ksys::cdevsw = {
            let mut c: ksys::cdevsw = unsafe { mem::zeroed() };
            c.d_open = Some(cdev_open::<T>);
            c.d_close = Some(cdev_close::<T>);
            c.d_read = Some(cdev_read::<T>);
            c.d_write = Some(cdev_write::<T>);
            c.d_version = ksys::D_VERSION as i32;
            c.d_name = "whatsthisstringfor".as_ptr() as *mut i8;
            Box::into_raw(box c)
        };

        let mut cdev_raw: *mut ksys::cdev = unsafe {
            ksys::make_dev(cdevsw_raw,
                           mem::transmute(0i32),
                           UID_ROOT,
                           GID_WHEEL,
                           0o660,
                           cstr_ref!(name).as_ptr() as *mut i8)
        };

        match cdev_raw.is_null() {
            true => {
                // Convert cdevsw back to Box so memory can be freed
                let cdevsw = unsafe { Box::from_raw(cdevsw_raw) };
                None
            }
            false => {
                let cdev = box CDev {
                                   cdev: unsafe { ptr::Unique::new(cdev_raw) },
                                   delegate: delegate,
                               };
                unsafe { (*cdev_raw).si_drv1 = mem::transmute(&*cdev) }
                Some(cdev)
            }
        }
    }
}
impl<T> fmt::Debug for CDev<T>
    where T: CharacterDevice
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CDev {{ cdev: {:?}, ... }}", self.cdev.as_ptr())
    }
}
impl<T> ops::Drop for CDev<T>
    where T: CharacterDevice
{
    fn drop(&mut self) {
        // debugln!("[kernel.rs] CDev::drop");

        // Assign only to clarify what type we're dealing with...
        let mut dev: *mut ksys::cdev = self.cdev.as_ptr();

        // Back to Box so cdevsw memory is freed
        let cdevsw: Box<ksys::cdevsw> = unsafe { Box::from_raw((*dev).si_devsw) };

        // debugln!("[kernel.rs] CDev::drop calling destroy_dev. ptr={:?}", dev.as_ptr());
        unsafe { ksys::destroy_dev(dev) };
    }
}

// File operations callbacks
extern "C" fn cdev_open<T>(dev: *mut ksys::cdev,
                           oflags: ::std::os::raw::c_int,
                           devtype: ::std::os::raw::c_int,
                           td: *mut ksys::thread)
                           -> ::std::os::raw::c_int
    where T: CharacterDevice
{
    // debugln!("cdev_open");
    let cdev: &CDev<T> = unsafe { mem::transmute((*dev).si_drv1) };
    if let Some(mut m) = cdev.delegate.lock() {
        m.open();
    }
    0
}

extern "C" fn cdev_fdopen(dev: *mut ksys::cdev,
                          oflags: ::std::os::raw::c_int,
                          td: *mut ksys::thread,
                          fp: *mut ksys::file)
                          -> ::std::os::raw::c_int {
    // debugln!("cdev_fdopen");
    0
}

extern "C" fn cdev_close<T>(dev: *mut ksys::cdev,
                            fflag: ::std::os::raw::c_int,
                            devtype: ::std::os::raw::c_int,
                            td: *mut ksys::thread)
                            -> ::std::os::raw::c_int
    where T: CharacterDevice
{
    // debugln!("cdev_close");
    let cdev: &CDev<T> = unsafe { mem::transmute((*dev).si_drv1) };
    if let Some(mut m) = cdev.delegate.lock() {
        m.close();
    }
    0
}

extern "C" fn cdev_read<T>(dev: *mut ksys::cdev,
                           uio: *mut ksys::uio,
                           ioflag: ::std::os::raw::c_int)
                           -> ::std::os::raw::c_int
    where T: CharacterDevice
{
    // debugln!("cdev_read");
    let cdev: &CDev<T> = unsafe { mem::transmute((*dev).si_drv1) };
    if let Some(mut m) = cdev.delegate.lock() {
        m.read(UioWriter::new(uio));
    }
    0
}

extern "C" fn cdev_write<T>(dev: *mut ksys::cdev,
                            uio: *mut ksys::uio,
                            ioflag: ::std::os::raw::c_int)
                            -> ::std::os::raw::c_int
    where T: CharacterDevice
{
    // debugln!("cdev_write");
    let cdev: &CDev<T> = unsafe { mem::transmute((*dev).si_drv1) };
    if let Some(mut m) = cdev.delegate.lock() {
        m.write(UioReader::new(uio));
    }
    0
}
