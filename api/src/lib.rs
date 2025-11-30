#![no_std]
#![feature(c_variadic)]
#![feature(thread_local)]

use core::{ffi::va_list::VaList, mem};
pub use api_sys as sys;
use core::ffi::{self, c_double, c_long};
use bitflags::bitflags;

#[macro_use]
pub mod macros;
pub mod arch;
pub mod errno;



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
    addr:*mut ffi::c_void,
    size: usize, // size_t
    proto:MMapProt,
    flags: MMapFlags,
    fd: ffi::c_int,
    offset:c_long
)->*mut ffi::c_void{
    
    const MMAP2:core::ffi::c_long = 192; // For some reason the sys crate isn't including
    // SYS_mmap2, but this is what the defined value is
    unsafe {syscall!(MMAP2,
                    addr as _,
                    size as _,
                    proto.bits() as _,
                    flags.bits() as _,
                    fd as _,
                    offset as _
                ) as _}
}

#[unsafe(no_mangle)]
pub extern "C" fn write(
    fd:ffi::c_int,
    buffer:*const ffi::c_void,
    size: usize

)->isize{ // ssize_t
    unsafe{syscall!(sys::SYS_write as _ ,
        fd as _,
        buffer as _,
         size as _) as _
        } 
}

fn abort()->!{
    syscall!(api_sys::SYS_kill)
}
fn getpid(){
    unsafe {syscall!(api_sys::SYS_getpid)}
}