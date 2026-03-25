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
    cfg_select,
    core_intrinsics,
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
// @TODO: have either the helper do more or write it's functionality in assembly, as we can optimize it, even slightly
// by inlining the logic 
"callq {call_main}",
// If it returns (it shouldn't) just invoke an invalid instruction
// We will use int 3, which is commonly used for debuggers as a breakpoint
// (Reason here:https://stackoverflow.com/questions/61816297/what-is-int-3-really-supposed-to-do)
// Which is useful to diagnosis if we did return
// This will also reliably raise SIGTRAP signal if no debugger (which will help reduce)
"int3", 
            call_main = sym start_main,
            options(att_syntax)
        }

    }
}
}

// There is no C++ support yet, so this function can never unwind
// That being said this should ALWAYS be statically linked
// but we cannot define that without defining 
unsafe extern "C"{
    unsafe fn main(argc:ffi::c_int, argv: *mut *mut ffi::c_char)-> ffi::c_int;
}
unsafe extern "C-unwind" {
    /// defined in [`api::rt`]. This allows the actual runtime be spined up outside of crt0
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
/// [`rslibc_start_entrypoint`]: __rslibc_start_entrypoint_1
// @TODO: just write it in assembly under the _start call since we are just calling the rslibc's entrypoint and passing all the values
// this can provide a small speed up since we reduce a unnecessary function call
#[inline(never)]
#[doc(hidden)] // internal detail
unsafe extern "C" fn start_main(argc:ffi::c_int, unbound_argv: *mut *mut ffi::c_char,envp: *mut *mut ffi::c_char)->!{
        __rslibc_start_entrypoint_1(main as _ ,unbound_argv,argc,envp);
    
}


#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test,expect(dead_code, reason="tests use their own panic handler"))]
// #[cfg_attr(test)]
#[cfg_attr(not(test),expect(dead_code, reason="tests use their own panic handler"))]
#[cfg_attr(not(test),expect(unused_attributes, reason = "Not exporting the symbol??"))]
#[inline(always)] // Find out why this code path was hit
fn panic_handler(_:&core::panic::PanicInfo) ->!{
    core::intrinsics::abort();
}


#[lang = "eh_personality"]
#[linkage = "weak"]
#[cfg(not(test))]
#[doc(hidden)]
// If not defined it will produce linker error when trying to link
// this staticlib 
// Rust will depend on this even if panic strategy is set to "abort" and not "unwind"
//https://users.rust-lang.org/t/unexpected-undefined-reference-to-rust-eh-personality-when-compiling-with-c-panic-abort-for-no-std-library/120311/2
// We can't use it as it will fail to even compile this crate as we would need to do -Zbuild-std=core but with that we can't use compiler builtins external crate
// with the sysroot one (that core depends on)
extern "C" fn rust_eh_personality() {}