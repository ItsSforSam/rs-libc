//! FFI which contains abstractions for the underlying OS
//! 
//! Similar to the [`std::ffi`]
#![expect(ambiguous_glob_reexports, reason = "We will be defining our own module")]
pub use core::ffi::*;
#[cfg(feature="alloc")]
pub use alloc::ffi::{CString,FromVecWithNulError,IntoStringError,NulError};

/// [`CStr`],[`CString`], and relating types
pub mod c_str{
    pub use core::ffi::c_str::*;
    pub use alloc::ffi::c_str::*;
}