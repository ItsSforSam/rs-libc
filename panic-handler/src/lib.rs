#![no_std]
#![allow(internal_features, reason = "Only used for core::intrinsics::abort. Which simply calls a invalid instruction")]
#![feature(core_intrinsics)]
use core::panic::PanicInfo;
use core::ffi;
use core::marker::PhantomData;



#[repr(C)]
pub struct RsLibCPanicPayload<'a>{
msg:*const ffi::c_char,
location:RsLibCPanicLocation,
_phantom:PhantomData<&'a ffi::CStr>
}
#[repr(C)]
pub struct RsLibCPanicLocation{

}
#[doc(hidden)] // Does not need to be generated in rustdoc
// SAFETY: 
#[unsafe(export_name = "__rslibc_panic")]
/// 
extern "C" fn panic_impl(_info:RsLibCPanicPayload)->!{
    
    loop {
        
    }
}
#[cfg_attr(not(test), panic_handler)]
fn call_panic(info:&PanicInfo) ->! {
    panic_impl(info.into())
}


impl<'a> From<&PanicInfo<'a>> for RsLibCPanicPayload<'a>{
    fn from(value: &PanicInfo) -> Self {
        let msg = value.message();
    }
}
#[unsafe(export_name = "__rslibc_abort_internal")]
#[no_mangle()]
extern "C" fn abort() -> !{
    core::intrinsics::abort()
}
