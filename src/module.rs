use std::boxed::Box;
use std::vec::Vec;
use std::sync::Arc;
use std::ops;
use std::cell::UnsafeCell;
use std::marker::Sync;
use std::cell::RefCell;
use std::string::String;
use std::io::{Read, Write};
use std::str;
use std::os::kernel as kern;

use spin::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

lazy_static! {
    // Object created on first access (which is module load callback)
    // NOTE: Wrap in Option so that we can take it out and free all memory at unload
    // (we could instead wrap all member in Option, which is best?)

    // XXX: If Arc<Mutex<Hello>>, anything in Arc<> will be leaked at kldunload!

    pub static ref MODULE: kern::SharedModule<Hello> = kern::SharedModule::new(Hello::new());
}


#[derive(Debug)]
pub struct HelloInner {
    data: Vec<u8>,
    cdev: Box<kern::CDev<Hello>>,
}


#[derive(Default, Debug)]
pub struct Hello {
    // Put everything in an option so that SharedModule<Hello> can be fully initialized
    // before we start doing stuff in module load callback.
    // (we can't for example clone MODULE while in Hello::new() because of order of initialization)
    inner: Option<HelloInner>,
}
impl Hello {
    fn new() -> Self {
        // We can't access MODULE here because it is not initialized yet!
        Hello { inner: None }
    }
}

impl kern::ModuleEvents for Hello {
    fn load(&mut self) {
        debugln!("[module.rs] Hello::load");

        // MODULE has been fully initialized here
        // so we can clone it safely
        let m = MODULE.clone();

        if let Some(cdev) = kern::CDev::new_with_delegate("rustmodule", m) {
            self.inner = Some(HelloInner {
                                  data: Vec::from("Default message :)\n"),
                                  cdev: cdev,
                              });
        } else {
            debugln!("[module.rs] Hello::load: Failed to create character device");
        }
    }

    fn unload(&mut self) {
        debugln!("[module.rs] Hello::unload");
    }
}

impl kern::CharacterDevice for Hello {
    fn open(&mut self) {
        // debugln!("[module.rs] Hello::open");
    }
    fn close(&mut self) {
        // debugln!("[module.rs] Hello::close");
    }
    fn read(&mut self, mut uio: kern::UioWriter) {
        // debugln!("[module.rs] Hello::read");

        if let Some(ref h) = self.inner {
            match uio.write_all(&h.data) {
                Ok(()) => (),
                Err(e) => debugln!("{}", e),
            }
        }
    }
    fn write(&mut self, mut uio: kern::UioReader) {
        // debugln!("[module.rs] Hello::write");

        let mut s = String::new();
        match uio.read_to_string(&mut s) {
            Ok(x) => debugln!("Read {} bytes. Setting new message to {:?}", x, s),
            Err(e) => debugln!("{:?}", e),
        }

        if let Some(ref mut h) = self.inner {
            h.data = Vec::from(s);
        }
    }
}
impl ops::Drop for Hello {
    fn drop(&mut self) {
        // debugln!("Hello::drop");
    }
}
