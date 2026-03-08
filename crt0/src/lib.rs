//! Starts up the C runtime
//! 
//! This package is intentionally minimal and internal.
//! Therefore most docs here are for developers primarily
#![no_main]
#![no_std]
#![feature(
    linkage,
    lang_items,
    abi_custom,
)]
// Only needed in test cfg, but we cannot optionally use features
#![allow(internal_features,reason="To shut rust up about no eh_personality")]

use core::ffi;
unsafe extern "custom"{
    /// The true starting point of the program
    /// 
    /// 
    /// This function simply pulls argv, argc and environment
    /// and calls the [`rslibc_start_entrypoint`].
    /// 
    /// This function may also do special handling 
    /// 
    /// 
    /// To retrieve the source, go to `crt0/src/arch/$TARGET_ARCH.asm`
    /// 
    /// [`rslibc_start_entrypoint`]: __rslibc_start_entrypoint_1
    unsafe fn _start();

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

// # Linking
// There is no C++ support yet, so this function can never unwind
unsafe extern "C"{
    unsafe fn main(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int;
}
unsafe extern "C-unwind" {
    /// defined in [`api::rt`]. This allows the actual runtime be spinned up outside of crt0
    /// 
    /// Entrypoints may need to have a signature to be changed, so there may be multiple potential entrypoints with different suffixes
    /// `__rslibc_start_entrypoint_#`
    safe fn __rslibc_start_entrypoint_1(
            mainfn:unsafe extern "C" fn(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int,
            argv:*mut *mut ffi::c_char,
            argc:ffi::c_int,
            envp: *mut *mut ffi::c_char
        )->!;
}
/// This simply allows [_start] to call the function and do any additional start up which is architecture agnostic.
/// But this is mostly handed off to [`rslibc_start_entrypoint`]
/// 
/// # SAFETY
/// Should never be called directly by rust
/// 
/// # ABI Breakage
/// This function is marked for INTERNAL USE ONLY, which means any use of it can lead to breaking changes
/// and not officially supported
/// 
/// [`rslibc_start_entrypoint`]: __rslibc_start_entrypoint_1
#[inline(never)]
#[doc(hidden)] // internal detail
#[unsafe(export_name = "__internal_start_main")]
pub unsafe extern "C" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char,envp: *mut *mut ffi::c_char)->!{
        __rslibc_start_entrypoint_1(main as _ ,unbound_argv,argc,envp);
    
}

//@TODO: Have PanicInfo equivalent be ffi safe
#[linkage="weak"] 
extern "C"  fn __panic_impl(_i:&core::panic::PanicInfo)->!{
    // SAFETY: while is UB, the actual behaver is defined on platforms. Segfault.
    // and if you are able to read at null, we have other issues
    // @TODO: mmap can techically be mapped at 0, making it valid.
    // See: https://wiki.debian.org/mmap_min_addr
    // Maybe have it automatically allow for it protecting a low range to prevent
    //  NULL-pointer privilege escalation if possible, maybe with mprotect?
    // But sometimes there are uses with using low ranges
    let _:u8 = unsafe {core::ptr::read_volatile(core::ptr::null())};
    // SAFETY: Should segfault
    unsafe {core::hint::unreachable_unchecked()}

unsafe extern "C" {
    safe fn __panic_impl(i:&core::panic::PanicInfo)->!;
}


#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test,expect(dead_code, reason="tests use their own panic handler"))]
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