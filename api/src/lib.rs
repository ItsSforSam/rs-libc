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
    const_trait_impl,
    // Allows the compiler
    // https://gist.github.com/joboet/0cecbce925ee2ad1ee3e5520cec81e30
    temporary_niche_types
)]
#![expect(internal_features,
    reason ="We will use them in the same way std/alloc/core use them, and will remove them if a stable version (or non internal version) presents itself")]
#[no_link]
pub extern crate api_sys as sys;
// use bitflags::bitflags;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod errno;
pub mod io;
pub mod mman;
pub mod ffi;
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





/// Aborts by calling signal
/// 
/// Currently if it fails it calls an invalid instruction via rust's [core::intrinsics::abort]
#[expect(unreachable_code, reason = "We are trying to kill this program no matter what. So we try multiple never functions, even if they siminly never return")]
pub fn abort()->!{
    // @TODO: unmask signal 
    let _ = kill(getpid(), api_sys::SIGABRT as _);

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
    unsafe {syscall!(SYS_getpid).unwrap_unchecked() as _}
}
pub fn kill(pid:i32, sig:c_int)->Result<c_int>{

    // SAFETY: passes correct prams 
   let v =unsafe { syscall!(SYS_kill,pid,sig)}?;
   Ok(v as c_int)
}
/// This is the equivalent to the [_exit(3p)]
/// 
/// Like glibc, on Linux systems this function calls [exit_group(2)] under the hood which terminates
/// all threads on the running process.
/// On all other Unix systems this currently just runs the normal exit syscall, but this may change
/// 
/// 
/// The value <code>status & 0xFF</code> is returned to the parent process as the process's exit status,
/// and can be collected by the parent using one of the [wait(2)] family of calls.
/// 
/// [_exit(3p)]: https://man.archlinux.org/man/exit.3p.en
/// [exit_group(2)]:https://man.archlinux.org/man/exit_group.2.en
/// [wait(2)]:https://man.archlinux.org/man/wait.2.en
#[unsafe(export_name = "_exit")]
pub extern "C" fn quick_exit(status:c_int) -> !{
    // SAFETY: proper parameters and type is passed
    #[cfg(target_os = "linux")]
    unsafe {syscall!(SYS_exit_group,status);}
    #[cfg(not(all(target_os = "linux",unix)))]
    unsafe {syscall!(SYS_exit,status);}
    #[cfg(not(unix))]
    todo!("Implement this for Windows target and other non-Unix"); // Should try to port to Windows as a whole
    // SAFETY: the syscall guarantees that the system has exited at this point
    unsafe {core::hint::unreachable_unchecked()}
}