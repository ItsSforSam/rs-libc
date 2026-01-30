use core::mem::MaybeUninit;
use core::ffi::{c_int, c_void};

type SAHandler = Option<extern "C" fn(c_int)>; 
type SASigaction = Option<extern "C" fn(c_int,*mut siginfo,*mut c_void)>;
/// Representation of the sigaction struct
#[repr(C)]
#[derive(Debug)]
pub struct sigaction{
    // The man page lists that sa_handler and sa_action
    // are sometimes a union and one must not set both
    // The actual kernel includes has them be a union only
    // if __i386__ is defined, which is only defined on x86_32 
    #[cfg(target_arch = "x86")]
    action:UnionSigAction,
    #[cfg(not(target_arch = "x86"))]
    sa_handler:SAHandler,
    #[cfg(not(target_arch = "x86"))]
    sa_sigaction:SASigaction,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>
}
#[cfg(target_arch = "x86")]
union UnionSigAction{
    sa_handler:*mut SAHandler,
    sa_sigaction:SASigaction

}
pub enum SigAction {
    
}
#[repr(C)]
pub struct siginfo{
 // @TODO
}
#[derive(Debug)]
pub struct sigset_t{
    // copied from libc crate
    #[cfg(target_pointer_width = "32")]
    __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}