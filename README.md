# Echo - a FreeBSD kernel module written Rust (WIP) #

## Requirements ##

- FreeBSD machine
- Rust nightly
- Xargo


I recommend using `rustup` for installing and handling different versions of Rust.

### Installation ###

Rust 

See [https://www.rustup.rs/](https://www.rustup.rs/)

Xargo 

```sh
$ cargo install xargo
```

Setting Rust nightly 

Do in crate root: 

```sh
$ rustup override set nightly
```


## Content ##

### Root Folder ###

This folder is the `Cargo` root for the module itself.


### Crates ###

Here are all the dependency crates that are not available on crates.io.

`core` is built and handled completely by `Xargo`.


#### libstd_unicode ####

Copied from Rust source code, unmodified.


#### libcollections ####

Copied from Rust source code, unmodified.


#### liballoc ####

Copied from Rust source code, unmodified.


#### liballoc_system ####

Copied from Rust source code.

Added `kern_malloc.rs` which is the FFI bindings to kernel's malloc functions.

Modified `lib.rs` to use said functions.


#### freebsd\_kernel\_std ####

A minimal implementation of std for the echo module. FFI bindings are generated on build. 
Safe bindings exists for things necessary for this code to run. 



## Build ##

Check `build.sh` script if you like to make any changes then run 

```sh
$ ./build.sh
```


## Load Module ##

If build was successful there should be a `hello.ko` file in the root folder. Load with 

```sh
$ sudo kldload ./hello.ko
```


## Usage ##

```sh
$ cat /dev/rustmodule  
Default Message :)  
$ echo Hello > /dev/rustmodule   
Read 6 bytes. Setting new message to "Hello\n"  
$ cat /dev/rustmodule  
Hello  
$ sudo kldunload hello  
```

# Warning #

If there are any bugs or faulty code the kernel will most likely hang. 

I recommend testing the module in a virtual machine.

It keeps your system safe and it makes it easier to develop and test. 

