use core;


#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "eh_unwind_resume"]
extern "C" fn eh_unwind_resume() {}


#[no_mangle]
#[lang = "panic_fmt"]
extern "C" fn panic_impl(args: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    use core::fmt::Write;
    use std::io::KernelDebugWriter;
    let mut writer = KernelDebugWriter {};

    print!("Panicked at '");
    // If this fails to write, just leave the quotes empty.
    let _ = writer.write_fmt(args);
    println!("', {}:{}", file, line);
    // Force a null pointer read to crash.
    unsafe {
        let _ = *(core::ptr::null::<i32>());
    }
    // If that doesn't work, loop forever.
    loop {}
}
