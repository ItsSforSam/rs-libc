#![allow(unsafe_op_in_unsafe_fn, reason ="these are simply reexporting them under the C abi")]
#![allow(clippy::missing_safety_doc, reason ="they are wrappers for the most part")]
use core::ffi::{c_long as long,c_ulong as ulong};
use crate::arch::current::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall0(a:long)->long{
    syscall0(a)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall1(a:long,b:long)->long{
    syscall1(a,b)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall2(a:long,b:long,c:long)->long{
    syscall2(a,b,c)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall3(a:long,b:long,c:long,d:long)->long{
    syscall3(a,b,c,d)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall4(a:long,b:long,c:long,d:long,e:long)->long{
    syscall4(a,b,c,d,e)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall5(a:long,b:long,c:long,d:long,e:long,f:long)->long{
    syscall5(a,b,c,d,e,f)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __syscall6(a:long,b:long,c:long,d:long,e:long,f:long,g:long)->long{
    syscall6(a,b,c,d,e,f,g)
}
/// A macro which calls a syscall on the system.
/// 
/// Similar to [syscall(2)] but returns the raw result and does not update [errno(3)].
/// Consult the man page
/// 
/// This returns the the raw result. 
/// # Safety
/// These don't validate parameters passed to the syscall
/// 
/// While syscalls are generally safe, depending on the syscall tho can be unsafe in certain contexts. See [signal-safety(7)]
/// 
/// 
/// [syscall(2)]      : https://man.archlinux.org/man/syscall.2.en
/// [signal-safety(7)]: https://man.archlinux.org/man/signal-safety.7.en
/// [errno(3)]        : https://man.archlinux.org/man/errno.3.en
#[macro_export]
macro_rules! syscall {
    ($sc:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall0($sc as _))
    };
    ($sc:expr,$a:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall1($sc as _,$a as _))
    };
    ($sc:expr,$a:expr,$b:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall2($sc as _,$a as _,$b as _))
    };

    ($sc:expr,$a:expr,$b:expr,$c:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall3($sc as _,$a as _,$b as _,$c as _))
    };

    ($sc:expr,$a:expr,$b:expr,$c:expr,$d:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall4($sc as _,$a as _,$b as _,$c as _,$d as _))
    };


    ($sc:expr,$a:expr,$b:expr,$c:expr,$d:expr,$e:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall5($sc as _,$a as _,$b as _,$c as _,$d as _,$e as _))
    };

    ($sc:expr,$a:expr,$b:expr,$c:expr,$d:expr,$e:expr,$f:expr) => {
        $crate::arch::common::__syscall_ret($crate::arch::current::syscall6($sc as _,$a as _,$b as _,$c as _,$d as _,$e as _,$f as _))
    };
}
#[doc(hidden)]
pub fn __syscall_ret(ret:long)->long{
    if ret > (-4096i64) {
        unsafe{crate::errno::ERRNO = -ret};
        return -1;
    }
    ret
}