#[macro_export]
macro_rules! cstr {
	($arg:expr) => (concat!($arg, '\x00'))
}
macro_rules! cstr_ref {
	($arg:expr) => (&format!("{}\x00", $arg))
}

#[macro_export]
macro_rules! print {
	// Static (zero-allocation) implementation that uses compile-time `concat!()` only
	($fmt:expr) => ({
		let msg = cstr!($fmt);
		let ptr = msg.as_ptr() as *const ::std::os::raw::c_char;
		unsafe {
			::std::os::kernel::uprintf(ptr);
		}
	});

	// Dynamic implementation that processes format arguments
	($fmt:expr, $($arg:tt)*) => ({
		use ::core::fmt::Write;
		use std::io::KernelDebugWriter;
		let mut writer = KernelDebugWriter {};
        writer.write_fmt(format_args!($fmt, $($arg)*)).unwrap();
	});
}

#[macro_export]
macro_rules! println {
	($fmt:expr)              => (print!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)+) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { print!($($arg)*) })
}

#[macro_export]
macro_rules! debugln {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { println!($($arg)*) })
}
