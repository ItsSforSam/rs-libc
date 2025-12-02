#![no_std]
#![feature(c_variadic)]
#![feature(thread_local)]
#![expect(internal_features, reason ="Uses abort intrinsic, which is safe")]
#![feature(core_intrinsics)]
use core::{ffi::va_list::VaList, mem};
use api_sys::__pid_t;
pub use api_sys as sys;
use core::ffi::*;
use bitflags::bitflags;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod errno;
pub mod stdio;


// # Safety
// Calling 
pub unsafe extern "C" fn syscall(_call:c_double,mut args:...) -> c_long{
    // @TODO make list
    let a:c_long;
    let b:c_long;
    let c:c_long;
    let d:c_long;
    let e:c_long;
    let f:c_long;
    let g:c_long;
    // Safety: We may just clobber
    unsafe{
        a=args.arg();
        b=args.arg();
        c=args.arg();
        d=args.arg();
        e=args.arg();
        f=args.arg();
        g=args.arg();

        crate::arch::current::syscall6(a, b, c, d, e, f, g)
    }
    
}

bitflags! {
    pub struct MMapProt: u32{
        const Exec  = sys::PROT_EXEC;
        const Read  = sys::PROT_READ;
        const Write = sys::PROT_WRITE;
        const None  = sys::PROT_NONE;
    }
}

bitflags! {
    pub struct MMapFlags: u32{
        const Shared         = sys::MAP_SHARED;
        const SharedValidate = sys::MAP_SHARED_VALIDATE;
        const Private        = sys::MAP_PRIVATE;
        const Bit32          = sys::MAP_32BIT;
        const Anonymous      = sys::MAP_ANONYMOUS;
        // Ignored as caused DDoS
        const DenyWrite      = sys::MAP_DENYWRITE;
        // Ignored
        const Executable     = sys::MAP_EXECUTABLE;

        const Fixed          = sys::MAP_FIXED;
        const FixedNoReplace = sys::MAP_FIXED_NOREPLACE;
        const GrowsDown      = sys::MAP_GROWSDOWN;
        const HugetLB        = sys::MAP_HUGETLB;
        const HugetLB_2MP    = sys::MAP_HUGE_2MB;
        const HugetLB_1GB    = sys::MAP_HUGE_1GB;

        const Locked         = sys::MAP_LOCKED;
        const NonBlock       = sys::MAP_NONBLOCK;
        const NoReserve      = sys::MAP_NORESERVE;
        const Stack          = sys::MAP_STACK;
        const Sync           = sys::MAP_SYNC;
        // const Uninitialized =   sys::MAP_UNINITIALIZED;
    }
}

pub fn mmap(
    addr:*mut c_void,
    size: usize, // size_t
    proto:MMapProt,
    flags: MMapFlags,
    fd: c_int,
    offset:c_long
)->*mut c_void{
    
    const MMAP2:core::ffi::c_long = 192; // For some reason the sys crate isn't including
    // SYS_mmap2, but this is what the defined value is
    unsafe {syscall!(MMAP2,
                    addr,
                    size,
                    proto.bits(),
                    flags.bits(),
                    fd,
                    offset
                ) as _}
}

#[unsafe(no_mangle)]
pub extern "C" fn write(
    fd:c_int,
    buffer:*const c_void,
    size: usize

)->isize{ // ssize_t
    unsafe{syscall!(sys::SYS_write,
        fd,
        buffer,
         size) as _
        } 
}

/// Aborts by calling signal
/// 
/// Currently if it fails it calls an invalid instruction via rust's
/// []
#[expect(unreachable_code, reason = "We are trying to kill this program no matter what. So we try multiple never functions, even if they siminly never return")]
pub fn abort()->!{
    // @TODO: unmask signal 
    kill(getpid(), api_sys::SIGABRT as _);

    core::intrinsics::abort();
    // SAFETY: Technically this is UB, but we are intentionally accessing an invalid address
    let _:u8 = unsafe {core::ptr::read_volatile(core::ptr::null())};
    unreachable!("Abort function failed. Panicking");
}
fn getpid() -> api_sys::__pid_t{
    unsafe {syscall!(api_sys::SYS_getpid) as _}
}
fn kill(pid:api_sys::__pid_t, sig:c_int)->c_int{
   unsafe { syscall!(api_sys::SYS_kill,pid,sig) as _}
}