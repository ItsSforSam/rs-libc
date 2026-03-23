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
    cfg_select
)]
// Only needed in test cfg, but we cannot optionally use features
#![allow(internal_features,reason="To shut rust up about no eh_personality")]
use compiler_builtins as _;
use core::ffi;
// unsafe extern "custom"{
//     
//     unsafe fn _start();

// }
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
#[unsafe(no_mangle)]
// #[linkage = "weak"]
#[unsafe(naked)]
unsafe extern "custom" fn _start(){
    cfg_select! {
    
    all(target_arch = "x86_64", target_os = "linux") =>{
        core::arch::naked_asm!{

"endbr64",
// .cfi_undefined %rip // prevent DWARF-based unwinders unwinding further
"pop %rdi", // argc
"mov %rsp, %rsi", // argv[]
"lea 8(%rsi,%rdi,8),%rdx", // then a null, then get the envp
// We want to re-aline the stack. Linux is very good with keeping the ABI compatible
// The issue is dynamic linkers, like musl's ldso which opts to no align the stack when explicitly invoked
// as noted her <https://github.com/ziglang/zig/blob/738d2be9d6b6ef3ff3559130c05159ef53336224/lib/std/start.zig//L240>
"xorl %ebp, %ebp", // zero stack frame
"and $-16, %rsp",  // have esp be 16 bits alined
// We call this helper due to us not needing to write EVERYTHING in assembly
"callq {call_main}",
// If it returns (it shouldn't) just invoke an invalid instruction
// Set's do hlt right here which is valid but requires ring0 access, which
// shouldn't occur, unless this code is running in kernel space which shouldn't be possible
"hlt",
//xorl %ebp, %ebp
//movq %rsp, %rdi
//andq $-16, %rsp
//callq __internal_start_main",
            call_main = sym start_main,
            options(att_syntax)
        }

    }
}
}
// //[cfg(target_arch = "x86")]
// global_asm!{
    
//     ".globl _start"

//     "_start:"
    
//     ,
//     options(att_syntax)
// }
// //[cfg(target_arch = "x86_64")]
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

// // Linking
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
pub unsafe extern "C" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char,envp: *mut *mut ffi::c_char)->!{
        __rslibc_start_entrypoint_1(main as _ ,unbound_argv,argc,envp);
    
}


unsafe extern "C-unwind" {
    // If it cannot be linked it will instead just dereference a NULL pointer and crash
    // the program
    #[linkage = "extern_weak"]
    //@TODO: Have PanicInfo equivalent be ffi safe"
     static __panic_impl: *const extern "C" fn(i:&core::panic::PanicInfo)->!;
}


#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test,expect(dead_code, reason="tests use their own panic handler"))]
// #[cfg_attr(test)]
#[cfg_attr(not(test),expect(dead_code, reason="tests use their own panic handler"))]
#[inline(never)] // Have it so it can be breakpoint in a debugger
fn panic_handler(info:&core::panic::PanicInfo) ->!{
    // SAFETY: We either deref a null pointer and segfault effectively crashing, or call the crash handler and crash
    // not safe, but crt0 is one of the few that shouldn't panic as we aren't doing anything to panic
    let impl_ = unsafe {*__panic_impl};
    impl_(info)
}


#[lang = "eh_personality"]
#[linkage = "weak"]
#[cfg(not(test))]
#[doc(hidden)]
pub extern "C" fn rust_eh_personality() {}