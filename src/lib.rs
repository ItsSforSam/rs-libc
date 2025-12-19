//! rslibc runtime
#![no_std]
#![no_main]
#![allow(internal_features, reason ="Uses lang item of eh_personality")]
#![feature(linkage,
    lang_items, // for lang item of eh_personality
    alloc_error_handler, 
    generic_atomic, // provides Atomics to be easier to read
    thread_local,   // used for errno and protection of double panic
)]
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
    loop{
        // If this hangs then it's being used correctly
        //@TODO: print version info and metadata
    }
}

/// Meta info for rslib allowed to be parsed by 
#[allow(warnings, reason = "Auto generated")]
mod meta{
    include!(concat!(env!("OUT_DIR"),"/meta.rs"));
}