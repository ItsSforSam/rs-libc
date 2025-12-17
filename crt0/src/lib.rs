//! Starts up the C runtime
#![no_main]
#![no_std]
#![feature(linkage)]
use core::ffi;
use core::arch::global_asm;

#[cfg(target_arch = "x86")]
global_asm!{
    
    ".globl _start"

    "_start:"
    
    ,
    options(att_syntax)
}
#[cfg(target_arch = "x86_64")]
global_asm!{
    ".globl _start",
    ".type _start, @function",
    "_start:",
        "endbr64",
        "call {libc_start}",
        
        "mov rdi, rax",

        "mov rax, 60",
        "syscall",
    // Exits program
    libc_start = sym start_main, // Allows name mangling and not exporting it out of obj file unnecessarily
}




/// # SAFETY
/// Should never be called directly by rust
pub unsafe extern "C" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char)->!{
    unsafe extern "C" {
        // Allows libc to call main as
        unsafe fn main(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int;
        /// def in [`api::rt`]

        #[link()]
        unsafe fn __rslibc_start_entrypoint_1(
            mainfn:unsafe extern "C" fn(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int,
            argv:*mut *mut ffi::c_char,
            argc:ffi::c_int
        )->!;
    }
    // SAFETY: There can be no safety gurantees as we are simply passing raw pointer to a foreign function
    unsafe {__rslibc_start_entrypoint_1(main as _,unbound_argv,argc)};
    
}



// We will weakly link to the main panic handler
#[linkage = "weak"]
#[expect(clippy::empty_loop,reason="Does not have stuff to implement cleanly")]
#[unsafe(no_mangle)]
pub extern "Rust" fn __panic_impl(_i:&core::panic::PanicInfo)->!{
    loop{
        
    }
}


#[cfg_attr(not(test),panic_handler)]
// #[cfg_attr(test)]
#[linkage = "weak"]
fn panic_handler(info:&core::panic::PanicInfo) ->!{
    __panic_impl(info)
}