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
        "call __libc_start_main",

    
    "mov rax, 60",
    "syscall"
    // Exits program
}


// SAFETY: No name clashes will occur
#[unsafe(export_name = "__libc_start_main")]
pub extern "C-unwind" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char)->ffi::c_int{
    unsafe extern "C" {
        // Allows libc to call main as
        fn main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char)-> ffi::c_char;
        
    }
    0
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