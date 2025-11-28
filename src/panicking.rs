
use core::panic::PanicInfo;

#[linkage = "weak"]
#[unsafe(no_mangle)]
pub extern "Rust" fn __panic_impl(_i:&PanicInfo)->!{
    todo!()
}

#[cfg_attr(not(test),panic_handler)]
#[cfg_attr(test, allow(unused))]
fn panic_handler(info:&PanicInfo) ->!{
    __panic_impl(info)
}