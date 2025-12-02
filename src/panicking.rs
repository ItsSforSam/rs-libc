//! Panicking runtime
//! 
//! The exact behavior of a panic should not be relied upon.
//! 
//! When a panic occurs it should be treated as a unresolvable error.
//! A backtrace may or may not be given.
//! 
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "Rust" fn __panic_impl(_i:&PanicInfo)->!{
    todo!()
}

#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test, allow(unused))]
fn panic_handler(info:&PanicInfo) ->!{
    __panic_impl(info)
}
// https://github.com/rust-lang/rust/blob/HEAD/library/std/src/sys/personality/gcc.rs
// If Rust supplies this, use it, but mostly us used to prevent ld to stop complaining
// that "rust_eh_personality" is not defined
#[lang = "eh_personality"]
#[linkage = "weak"]
#[cfg(not(test))]
#[doc(hidden)]
pub extern "C" fn rust_eh_personality() {}

// Safety: has special prefix
#[unsafe(no_mangle)]
#[alloc_error_handler]
pub fn __rslibc_oom_error(_: core::alloc::Layout)->!{
    todo!()

}