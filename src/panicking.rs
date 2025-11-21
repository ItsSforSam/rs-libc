
use core::panic::PanicInfo;
pub extern "C-unwind" fn __panic_impl(){

}

#[panic_handler]
fn panic_handler(_info:&PanicInfo) ->!{
    __panic_impl()
}