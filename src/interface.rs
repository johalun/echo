use std::os::kernel::ModuleEvents;
use std::os::kernel;
use std::os::raw;
use std::marker::Sync;
use module::MODULE;
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex};


#[no_mangle]
pub extern "C" fn module_event(module: kernel::Module,
                               event: raw::c_int,
                               arg: *mut raw::c_void)
                               -> raw::c_int {
    // debugln!("[interface.rs] Got event {}", event);

    if let Some(ev) = kernel::ModuleEventType::from_i32(event) {
        match ev {
            kernel::ModuleEventType::MOD_LOAD => {
                // debugln!("[interface.rs] MOD_LOAD");

                if let Some(mut m) = MODULE.lock() {
                    m.load();
                }
            }
            kernel::ModuleEventType::MOD_UNLOAD => {
                // debugln!("[interface.rs] MOD_UNLOAD");

                if let Some(mut m) = MODULE.lock() {
                    m.unload();
                }

                MODULE.cleanup();
            }
            kernel::ModuleEventType::MOD_QUIESCE => {
                // debugln!("[interface.rs] MOD_QUIESCE");
            }
            kernel::ModuleEventType::MOD_SHUTDOWN => {
                // debugln!("[interface.rs] MOD_SHUTDOWN");
            }
        }
    } else {
        debugln!("[interface.rs] Undefined event");
    }
    0
}
