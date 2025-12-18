//! rslibc runtime
#![no_std]
#![no_main]
#![allow(internal_features, reason ="Uses lang item of eh_personality")]
#![feature(linkage,lang_items,alloc_error_handler,generic_atomic,thread_local)]
mod panicking;
mod rt;
pub mod env;
extern crate alloc;

#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator::new();

/// This is the "entrypoint" to the dynamic object.
/// 
/// This allows providing debug info easily for scripts
/// 
/// This is set by the build script
#[unsafe(no_mangle)]
pub extern "C" fn __libc_main()->!{
    todo!("Implement")
}