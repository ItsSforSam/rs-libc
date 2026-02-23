#![no_std]
//! Allows calling syscalls for Linux and other platforms
#[doc(hidden)]
pub extern crate api_sys as sys;

pub mod arch;
pub mod errno;

