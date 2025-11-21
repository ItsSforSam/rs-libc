#![no_std]
// #![feature(c_variadic)]
use api_sys as sys;
use core::ffi::{self, c_double, c_long};
#[macro_use]
pub mod macros;

#[cfg(target_arch="x86_64")]
#[path ="arch/x86-64.rs"]
pub mod arch;
#[path ="arch/common.rs"]
pub mod arch_common;
// SAFETY
// Calling 
pub unsafe extern "C" fn syscall(call:c_double,mut args:VaListImpl<'_>) -> c_long{
    let a:c_long;
    let b:c_long;
    let c:c_long;
    let d:c_long;
    let e:c_long;
    let f:c_long;
    unsafe{
        a=args.arg();
        b=args.arg();
        d=args.arg();
        e=args.arg();
        f=args.arg();
    }
    todo!()
}


pub fn write(){

}

