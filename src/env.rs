//! Handles environment variables
//! 
//! An

use core::ffi::{CStr, c_char};

/// This allows lazily be placed in 
#[derive(Debug)]
pub struct EnvVars{

    
}

#[expect(clippy::new_without_default)]
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