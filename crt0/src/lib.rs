//! Starts up the C runtime
#![no_main]
#![no_std]
#![feature(
    linkage,
    lang_items,
    abi_custom,
)]
#![expect(internal_features,reason="To shut rust up about no eh_personality")]

use core::ffi;
use core::arch::global_asm;

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[cfg(target_arch = "x86_64")]
unsafe extern "custom" fn _start() ->!{
    core::arch::naked_asm!{
        "pop %rdi",
        "call {libc_start}",
        // If it returns (it shouldn't just invoke an invalid instruction)
        // Set's do hlt right here which is valid but requires ring0 access
        "hlt",
        libc_start = sym start_main,
        
    }
}

// #[cfg(target_arch = "x86")]
// global_asm!{
    
//     ".globl _start"

//     "_start:"
    
//     ,
//     options(att_syntax)
// }
// #[cfg(target_arch = "x86_64")]
// global_asm!{
//     ".globl _start",
//     ".type _start, @function",
//     "_start:",
//         "endbr64",
        
//         "call {libc_start}",
        

//         "hlt",
//     // Exits program
//     libc_start = sym start_main, // Allows name mangling and not exporting it out of obj file unnecessarily
// }




/// # SAFETY
/// Should never be called directly by rust
// #[cfg_attr(predicate, attr)]
#[inline(never)]
pub unsafe extern "C" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char)->!{
    unsafe extern "C-unwind" {
        // Allows libc to call main as
        unsafe fn main(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int;
        /// def in [`api::rt`]
        safe fn __rslibc_start_entrypoint_1(
            mainfn:unsafe extern "C-unwind" fn(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int,
            argv:*mut *mut ffi::c_char,
            argc:ffi::c_int
        )->!;
    }
        __rslibc_start_entrypoint_1(main as _,unbound_argv,argc);
    
}

//@TODO: Have PanicInfo equivalent be ffi safe
unsafe extern "C" {
    safe fn __panic_impl(i:&core::panic::PanicInfo)->!;
}


#[cfg_attr(not(test),panic_handler)]
// #[cfg_attr(test)]
#[cfg_attr(not(test),expect(dead_code, reason="tests use their own panic handler"))]
#[inline(never)] // Have it so it can be breakpoint in a debugger
fn panic_handler(info:&core::panic::PanicInfo) ->!{
    __panic_impl(info)
}


#[lang = "eh_personality"]
#[linkage = "weak"]
#[cfg(not(test))]
#[doc(hidden)]
pub extern "C" fn rust_eh_personality() {}