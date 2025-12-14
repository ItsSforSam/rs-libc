//! Handles environment variables
//! 
//! Holds environment information
#[cfg(target_has_atomic="ptr")]
use core::sync::atomic::{AtomicIsize,AtomicPtr,Ordering,Atomic};

static ARGC:AtomicIsize = AtomicIsize::new(0);
static ARGV:Atomic<*mut *const u8> = AtomicPtr::new(core::ptr::null_mut());

// static ARGV_ARRAY: extern "C" fn(core::ffi::c_int)


use core::ffi::{CStr, c_char};

/// This allows lazily be placed in 
#[derive(Debug)]
pub struct EnvVars{

    
}

impl EnvVars{
    pub const fn new()->Self{
        EnvVars {
            
        }
    }

    unsafe fn from_ptr(ptr:*mut *mut c_char) ->Self{
        todo!()
    }

    
}


// SAFETY: Non
#[unsafe(export_name = "__environ")]
#[linkage = "weak"]
// #[alias = "_environ"]
pub(crate) static mut RAW_ENVIRON: *mut *mut c_char = core::ptr::null_mut();