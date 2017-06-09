
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

use spin::{Mutex, MutexGuard};

use super::kernel_sys as ksys;

use super::kernel::*;


// #[derive(Default)]
pub struct UioReader {
    uio: ptr::Unique<ksys::uio>,
    offset: usize,
    remain: usize,
}
impl UioReader {
    pub fn new(uio: *mut ksys::uio) -> Self {
        UioReader {
            uio: unsafe { ptr::Unique::new(uio) },
            offset: 0,
            remain: unsafe { (*(*uio).uio_iov).iov_len },
        }
    }
    fn len(&self) -> usize {
        unsafe { self.uio.as_ref().uio_resid as usize }
    }
}
impl io::Read for UioReader {
    // A reader is implemented for reading data from userland to kernel.
    // That is, for d_write callback.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {

        let buf_len = buf.len();
        let iov_len = unsafe { (*self.uio.as_ref().uio_iov).iov_len } - self.offset as usize;
        let len = cmp::min(buf_len, iov_len);

        if len == 0 {
            return Ok(0);
        }

        if buf_len < iov_len {
            // Still got some data to read
            self.remain = iov_len as usize - buf_len as usize;
        } else {
            // We read everything already
            self.remain = 0;
        }

        // Change to uiomove?
        let ret = unsafe {
            ksys::copyin((*self.uio.as_ref().uio_iov)
                             .iov_base
                             .offset(self.offset as isize),
                         buf.as_mut_ptr() as *mut raw::c_void,
                         len)
        };
        self.offset += len as usize;

        match ret {
            0 => Ok(len),
            _ => Err(io::Error::new(io::ErrorKind::Other, "UioReader::read copyin failed.")),
        }
    }
}
impl fmt::Debug for UioReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "UioReader {{ uio: {:?}, offset: {}, remain: {} }}",
               self.uio.as_ptr(),
               self.offset,
               self.remain)
    }
}



pub struct UioWriter {
    uio: ptr::Unique<ksys::uio>,
}
impl UioWriter {
    pub fn new(uio: *mut ksys::uio) -> Self {
        UioWriter { uio: unsafe { ptr::Unique::new(uio) } }
    }
    fn len(&self) -> usize {
        unsafe { self.uio.as_ref().uio_resid as usize }
    }
}
impl io::Write for UioWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Temporary add a uiomove function that takes immutable buffer instead of mutable
        extern "C" {
            pub fn uiomove(cp: *const ::std::os::raw::c_void,
                           n: ::std::os::raw::c_int,
                           uio: *mut ksys::uio)
                           -> ::std::os::raw::c_int;
        }

        let offset = unsafe { self.uio.as_ref().uio_offset as usize };
        let amount_uio = unsafe { self.uio.as_ref().uio_resid as usize };
        let amount_buf = match buf.len() - offset {
            x if x > 0 => x,
            _ => 0,
        };
        // debugln!("===> offest {}, amount uio {}, amount buf {}", offset, amount_uio, amount_buf);

        let amount = cmp::min(amount_buf, amount_uio);
        if amount == 0 {
            // return offset here so write_all know that we've already read all bytes.
            return Ok(offset);
        }

        let ret = unsafe {
            uiomove(buf[offset as usize..].as_ptr() as *const raw::c_void,
                    amount as i32,
                    self.uio.as_ptr())
        };
        match ret {
            0 => Ok(amount),
            _ => {
                Err(io::Error::new(io::ErrorKind::Other,
                                   format!("uiomove failed with return code {}", ret)))
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        // XXX What do we do here?
        Ok(())
    }
}
impl fmt::Debug for UioWriter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UioWriter {{ uio: {:?} }}", self.uio.as_ptr())
    }
}
