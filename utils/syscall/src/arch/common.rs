//! Common code that is architecture-agnostic
//! 
//! This contains the [`syscall!()`] macro
//! 
//! [`syscall!()`]:crate::syscall
#![allow(unsafe_op_in_unsafe_fn, reason ="these are simply reexporting them under the C abi")]
#![allow(clippy::missing_safety_doc, reason ="they are wrappers for the most part")]
#![allow(missing_docs, reason ="wrapper around rust functions to be exported")]
use core::ffi::c_long as long;
use crate::{errno::Errno};

/// A macro which calls a syscall on the system.
/// 
/// Similar to [syscall(2)] but returns the raw result and does not update [errno(3)].
/// Consult the man page
/// 
/// This returns the the raw result. 
/// # Safety
/// These don't validate parameters passed to the syscall
/// 
/// Ensure the proper type is passed and proper order.
/// 
/// While syscalls are generally "safe" if passed correct non malformed parameters, depending on the syscall tho can be unsafe in certain contexts. See [signal-safety(7)]
/// 
/// 
/// 
/// [syscall(2)]: <https://man.archlinux.org/man/syscall.2.en>
/// [signal-safety(7)]: <https://man.archlinux.org/man/signal-safety.7.en>
/// [errno(3)]: <https://man.archlinux.org/man/errno.3.en>
#[macro_export]
macro_rules! syscall {
    ($sc:ident) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall0($crate::sys::$sc as ::core::ffi::c_long))
    };
    ($sc:ident,$a:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall1($crate::sys::$sc as ::core::ffi::c_long,$a as _))
    };
    ($sc:ident,$a:expr,$b:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall2($crate::sys::$sc as ::core::ffi::c_long,$a as _,$b as _))
    };

    ($sc:ident,$a:expr,$b:expr,$c:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall3($crate::sys::$sc as ::core::ffi::c_long,$a as _,$b as _,$c as _))
    };

    ($sc:ident,$a:expr,$b:expr,$c:expr,$d:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall4($crate::sys::$sc as ::core::ffi::c_long,$a as _,$b as _,$c as _,$d as _))
    };


    ($sc:ident,$a:expr,$b:expr,$c:expr,$d:expr,$e:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall5($crate::sys::$sc as ::core::ffi::c_long,$a as _,$b as _,$c as _,$d as _,$e as _))
    };

    ($sc:ident,$a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall6($crate::sys::$sc as ::core::ffi::c_long,$a as _,$b as _,$c as _,$d as _,$e as _,$f as _))
    };
}
#[doc(hidden)]
#[inline]
pub fn __syscall_ret(ret:long)->Result<long,crate::errno::Errno>{
    
    // Some syscalls return large values, like lseek
    // But Linus says non errno returns won't be between -1 and -4095
    if (-4095..=-1).contains(&ret)  {
        return Err(Errno::try_from(-ret).unwrap());
    }
    
    Ok(ret)
}
/// Allows for converting the [Ok] to desired value.
/// Used instead of 
/// ```Rust,no_test
/// 
/// match syscall!(SYS_getpid){ // Note: getpid can not error, but is okay in this example
///     Ok(v) => Ok(v as _)
///     Err(v) => Err(v)
/// }
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! __syscall_convert_to_Result {
    ($e:expr) => {
        match $e {
            ::core::result::Result::Ok(v) => Ok(v as _)
            ::core::result::Result::Err(v) => Err(v)
        }
    };
}

