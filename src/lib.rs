#![no_std]
#![no_main]
#![feature(linkage)]
mod panicking;
mod rt;
pub mod env;
extern crate alloc;