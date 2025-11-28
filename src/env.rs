//! Handles environment variables
//! 
//! An

use core::ffi::CStr;

/// This allows lazily be placed in 
#[derive(Debug)]
struct EnvVars{

    // data:alloc::collections::Mutex<alloc::vec::Vec<CStr>>
}

impl EnvVars{
    pub const fn new()->Self{
        EnvVars {
            // data: Vec::new()
        }
    }
}