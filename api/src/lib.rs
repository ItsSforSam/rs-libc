#![no_std]
#![feature(c_variadic)]
use core::ffi::va_list::VaList;
use api_sys as sys;
use core::ffi::{self, c_double, c_long};


#[macro_use]
pub mod macros;
pub mod arch;



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


pub fn mmap(
    addr:*mut ffi::c_void,
    size: usize, // size_t
    proto:ffi::c_int,
    flags: ffi::c_int,
    fd: ffi::c_int,
    offset:c_long
)->*mut ffi::c_void{
    const MMAP2:core::ffi::c_long = 192; // For some reason the sys crate isn't including
    // SYS_mmap2, but this is what the defined value is
    unsafe {syscall!(MMAP2,
                    addr as _,
                    size as _,
                    proto as _,
                    flags as _,
                    fd as _,
                    offset as _
                ) as _}
}


extern "C" fn write(
    fd:ffi::c_int,
    buffer:*const ffi::c_void,
    size: usize

)->isize{ // ssize_t
    todo!()
}

