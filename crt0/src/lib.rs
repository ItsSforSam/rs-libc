//! Starts up the C runtime
//! 
//! This package is intentionally minimal and internal.
//! Therefore most docs here are for developers primarily
#![no_main]
#![no_std]
#![feature(
    linkage,
    abi_custom,
    cfg_select,
    core_intrinsics,
)]
// Only needed in test cfg, but we cannot optionally use features
#![allow(internal_features,reason="To allow use of core::intrinsics::abort in panic handler")]
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
#[link(name = "rs_libc")] // TODO: Have this properly work with just "C"
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

#[linkage="weak"]
#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test, expect(dead_code, reason = "No panic hander in tests"))]
fn panic_handler(_:&core::panic::PanicInfo) ->!{
    core::intrinsics::abort();
}

/// This allows crt0 to compile and link correctly as for some reason, in static builds
/// It refers to the mem builtin functions, despite core and use using the compiler_builtins crate
macro_rules! stub_builtin {
    (
        $(pub fn $func:ident$prams:tt $(-> $output:ty)?;)+
    ) => {
        $(
             
            #[linkage = "weak"]
            #[unsafe(no_mangle)]
            #[doc(hidden)]
            #[expect(unused, reason ="Meant to be overridden")]
            pub unsafe extern "C" fn $func$prams $(-> $output)*{
                ::core::unreachable!("{} not defined but is called", ::core::stringify!($func))
            }
        )*
    };
}

stub_builtin!{
    pub fn bcmp(s1:*const u8, s2:*const u8, n:usize) -> i32;
    pub fn memcmp(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memcpy(dest: *mut u8, src:*const u8,n:usize) -> *mut u8;
    pub fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    pub fn memset(dest: *mut u8, src:*const u8,n:usize);
    pub fn strlen(s:*const core::ffi::c_char)->usize;
    
}