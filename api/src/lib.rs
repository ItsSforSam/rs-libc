//! The api of the crate
//! 
//! These allow calling syscalls to the system (currently only Linux)
#![no_std]
#![feature(
    // Used for the abort intrinsics, which is safe to call, just
    // has unstable behavior (which on the currently pinned version causes a seg fault)
    // @TODO: implement abort the way it is actually supposed 
    core_intrinsics,
    c_variadic,
    thread_local,
    // Allows the compiler
    // https://gist.github.com/joboet/0cecbce925ee2ad1ee3e5520cec81e30
    temporary_niche_types
)]
#![expect(internal_features,
    reason ="We will use them in the same way std/alloc/core use them, and will remove them if a stable version (or non internal version) presents itself")]

extern crate api_sys as sys;
// use bitflags::bitflags;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod errno;
pub mod io;
pub mod mman;
pub mod ffi;
// Same as std::sys, but 
pub(crate) mod system;
pub mod os;
#[cfg(feature="alloc")]
extern crate alloc;
/// Most syscalls can fail with a errno value
/// 
/// This provides an alias that can be used when interacting with this crate
pub type Result<T> = core::result::Result<T, crate::errno::Errno>;
/// Equivalent to C's `size_t` type, from `stddef.h`.
/// 
/// Equivalent of Rust's [`core::ffi::c_size_t`] but not nightly
#[expect(non_camel_case_types, reason = "Mimics rust type alias")]
pub type c_ssize_t = isize;

/// Equivalent to C's `size_t` type, from `stddef.h`.
/// 
/// Equivalent of Rust's [`core::ffi::c_ssize_t`] but not nightly
#[expect(non_camel_case_types, reason = "Mimics rust type alias")]
pub type c_size_t = usize;
pub mod prelude{
    /// Due to the amount of ffi to C, it should be convenient.
    /// All ffi types are prefixed anyway
    pub use core::ffi::{
        c_char,c_double,c_uint,c_int,c_ulong,c_long,c_void
    };
    pub use crate::{c_size_t,c_ssize_t};
    pub use crate::errno::{Errno,ERRNO};
}
use prelude::*;
// # Safety
// Calling 
// pub unsafe extern "C" fn syscall(_call:c_long,mut args:...) -> c_long{
//     // @TODO make list
//     let a:c_long;
//     let b:c_long;
//     let c:c_long;
//     let d:c_long;
//     let e:c_long;
//     let f:c_long;
//     let g:c_long;
//     // Safety: We may just clobber
//     unsafe{
//         a=args.arg();
//         b=args.arg();
//         c=args.arg();
//         d=args.arg();
//         e=args.arg();
//         f=args.arg();
//         g=args.arg();
//         crate::arch::current::syscall6(a, b, c, d, e, f, g)
//     }
    
// }





/// Aborts by calling signal
/// 
/// Currently if it fails it calls an invalid instruction via rust's [core::intrinsics::abort]
#[expect(unreachable_code, reason = "We are trying to kill this program no matter what. So we try multiple never functions, even if they siminly never return")]
pub fn abort()->!{
    // @TODO: unmask signal 
    kill(getpid(), api_sys::SIGABRT as _);

    core::intrinsics::abort();
    // SAFETY: Technically this is UB, but we are intentionally accessing an invalid address. But most
    // OSes will respond with Seg fault. It technically is possible via mmap to have valid data there
    // (but if you do that, your shooting yourself in the foot anyway) ¯\_(ツ)_/¯
    let _:u8 = unsafe {core::ptr::read_volatile(core::ptr::null())};
    
    unreachable!("Abort function failed. No coredump generated");
}
pub fn getpid() -> i32{
    // SAFETY: Provides required params (none)
    // SAFETY: getpid does not error
    unsafe {syscall!(api_sys::SYS_getpid).unwrap_unchecked() as _}
}
pub fn kill(pid:i32, sig:c_int)->Result<c_int>{

    // SAFETY: passes correct prams 
   let v =unsafe { syscall!(api_sys::SYS_kill,pid,sig)}?;
   Ok(v as c_int)
}