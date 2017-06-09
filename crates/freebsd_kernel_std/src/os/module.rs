
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

pub trait ModuleEvents {
    fn load(&mut self);
    fn unload(&mut self);
}

impl ModuleEventType {
    pub fn from_i32(n: i32) -> Option<ModuleEventType> {
        match n {
            0 => Some(ModuleEventType::MOD_LOAD),
            1 => Some(ModuleEventType::MOD_UNLOAD),
            2 => Some(ModuleEventType::MOD_SHUTDOWN),
            3 => Some(ModuleEventType::MOD_QUIESCE),
            _ => None,
        }
    }
}

pub struct LockedModule<'a, T: Sized + 'a> {
    guard: MutexGuard<'a, Option<T>>,
}
impl<'a, T: Sized> LockedModule<'a, T> {}
impl<'a, T: Sized> Deref for LockedModule<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.guard.as_ref().unwrap()
    }
}
impl<'a, T: Sized> DerefMut for LockedModule<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.guard.as_mut().unwrap()
    }
}
impl<'a, T> ops::Drop for LockedModule<'a, T> {
    fn drop(&mut self) {
        // debugln!("[kernel.rs] LockedModule::drop");
    }
}
impl<'a, T> fmt::Debug for LockedModule<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LockedModule {{ guard: MutexGuard<Option<T>> }}")
    }
}

#[derive(Debug, Default)]
pub struct SharedModule<T> {
    inner: Arc<Mutex<Option<T>>>,
}
impl<T> SharedModule<T> {
    pub fn new(data: T) -> Self {
        SharedModule { inner: Arc::new(Mutex::new(Some(data))) }
    }
    pub fn clone(&self) -> Self {
        SharedModule { inner: self.inner.clone() }
    }
    pub fn inner(&self) -> Arc<Mutex<Option<T>>> {
        self.inner.clone()
    }
    pub fn lock(&self) -> Option<LockedModule<T>> {
        let guard = self.inner.lock();
        if guard.is_some() {
            Some(LockedModule { guard: guard })
        } else {
            None
        }
    }
    pub fn cleanup(&self) {
        {
            let _ = self.inner.lock().take();
        }
        // Safe to do this in kldunload callback?
        // If we don't, we'll leak 64 byte Mutex struct (maybe not a disaster...)
        unsafe {
            let ptr: *mut Arc<Mutex<Option<T>>> = mem::transmute(&self.inner);
            ptr::drop_in_place(ptr);
        }
    }
}
impl<T> ops::Drop for SharedModule<T> {
    fn drop(&mut self) {
        // debugln!("[kernel.rs] SharedModule::drop");
    }
}


unsafe impl<T> Sync for SharedModule<T> {}
unsafe impl<T> Send for SharedModule<T> {}
