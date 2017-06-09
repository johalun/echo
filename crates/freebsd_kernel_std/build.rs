extern crate bindgen;

use std::env;
use std::path::PathBuf;

const FILEPATH_CODE: &'static str = "src/os/kernel_sys.rs";

    
fn main() {

    env::set_var("LIBCLANG_PATH", "/usr/local/llvm40/lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        // .no_unstable_rust()
        .use_core()
        // Use this prefix instead of ::std::os::raw
        // .ctypes_prefix("raw")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Clang arguments
        .clang_arg("-O2")
        .clang_arg("-pipe")
        .clang_arg("-fno-strict-aliasing")
        .clang_arg("-Werror")
        .clang_arg("-D_KERNEL")
        .clang_arg("-DKLD_MODULE")
        .clang_arg("-nostdinc")
        .clang_arg("-I.")
        .clang_arg("-I/usr/src/sys")
        .clang_arg("-fno-common")
        .clang_arg("-fno-omit-frame-pointer")
        .clang_arg("-mno-omit-leaf-frame-pointer")
        .clang_arg("-MD")
        .clang_arg("-mcmodel=kernel")
        .clang_arg("-mno-red-zone")
        .clang_arg("-mno-mmx")
        .clang_arg("-mno-sse")
        .clang_arg("-msoft-float")
        .clang_arg("-fno-asynchronous-unwind-tables")
        .clang_arg("-ffreestanding")
        .clang_arg("-fwrapv")
        .clang_arg("-fstack-protector")
        .clang_arg("-Wall")
        .clang_arg("-Wredundant-decls")
        .clang_arg("-Wnested-externs")
        .clang_arg("-Wstrict-prototypes")
        .clang_arg("-Wmissing-prototypes")
        .clang_arg("-Wpointer-arith")
        .clang_arg("-Winline")
        .clang_arg("-Wcast-qual")
        .clang_arg("-Wundef")
        .clang_arg("-Wno-pointer-sign")
        .clang_arg("-D__printf__=__freebsd_kprintf__")
        .clang_arg("-Wmissing-include-dirs")
        .clang_arg("-fdiagnostics-show-option")
        .clang_arg("-Wno-unknown-pragmas")
        .clang_arg("-Wno-error-tautological-compare")
        .clang_arg("-Wno-error-empty-body")
        .clang_arg("-mno-aes")
        .clang_arg("-mno-avx")
        .clang_arg("-std=iso9899:1999")

        // Command for building kernel modulde
// cc  -O2 -pipe  -fno-strict-aliasing -Werror -D_KERNEL -DKLD_MODULE -nostdinc   -I. -I/usr/src/sys -fno-common  -fno-omit-frame-pointer -mno-omit-leaf-frame-pointer  -MD  -MF.depend.hello.o -MThello.o -mcmodel=kernel -mno-red-zone -mno-mmx -mno-sse -msoft-float  -fno-asynchronous-unwind-tables -ffreestanding -fwrapv -fstack-protector -Wall -Wredundant-decls -Wnested-externs -Wstrict-prototypes -Wmissing-prototypes -Wpointer-arith -Winline -Wcast-qual -Wundef -Wno-pointer-sign -D__printf__=__freebsd_kprintf__ -Wmissing-include-dirs -fdiagnostics-show-option -Wno-unknown-pragmas -Wno-error-tautological-compare -Wno-error-empty-body -Wno-error-parentheses-equality -Wno-error-unused-function -Wno-error-pointer-sign -Wno-error-shift-negative-value  -mno-aes -mno-avx  -std=iso9899:1999 -c hello.c -o hello.o

        
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(FILEPATH_CODE);
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
