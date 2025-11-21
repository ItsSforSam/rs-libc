//! Starts up the C runtime
#![no_main]
#![no_std]
use core::ffi::{c_char,c_int};


type MainFn = extern "C" fn(c_int,*mut *mut c_char) -> c_int;

// SAFETY: No name clashes will occur
#[unsafe(export_name = "__libc_start_main")]
extern "C" fn start_main(main:MainFn,argc:c_int, unbound_argv: *mut *mut c_char)->c_int{
    0
}