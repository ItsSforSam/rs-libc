//! FFI which contains abstractions for the underlying OS
//! 
//! Similar to the [`std::ffi`]
//! 
pub use core::ffi::*;
#[cfg(feature="alloc")]
mod c_str;
