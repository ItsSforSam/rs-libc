//! rslibc runtime
#![no_std]
#![no_main]
#![expect(internal_features, reason ="Uses lang item of eh_personality")]
#![feature(linkage,
    lang_items, // for lang item of eh_personality
    alloc_error_handler, 
    generic_atomic, // provides Atomics to be easier to read
    thread_local,   // used for errno and protection of double panic
    likely_unlikely,
    const_trait_impl, // Allows to impl traits in const fashion 
    const_convert,    // And here use said traits in const scopes, ig. Yes we need both
    negative_impls,
)]
use compiler_builtins as _; // Exposes required functions for C which are expected, will raise linker errors
mod panicking;
mod rt;
pub mod env;
pub mod stdio;
// mod mem;
extern crate alloc;
#[no_link]
extern crate cfg_if;
#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator::new();

/// This is the "entrypoint" to the dynamic object.
/// 
/// This allows providing information like copyright, version info, and where to report bugs
/// 
#[doc = include_str!("../meta/docs/abi-breakage.md")]
/// 
/// # Safety
/// This function won't produce undefined behavior when called, BUT is not supposed to be called explicitly, and is simply
/// used as a entry point for dynamic libraries and shouldn't be called directly
#[unsafe(no_mangle)]
#[cfg(linkage = "dynamic")]
pub unsafe extern "C" fn __libc_main()->!{
    loop{
        // If this hangs then it's being set correctly
        // as if a entry isn't set (and `_start` doesn't exist)
        // it sets to NULL, which just seg faults.
        // BUT cannot use todo!() as that just panics, which just calls abort (which is fine)
        // which doesn't have a stable result)
        //@TODO: print version info and metadata
        core::hint::spin_loop();
    }
}
/// Same as C's exit function
#[expect(unreachable_code, reason = "marked with todo")]
pub fn exit()->!{
    todo!("Implement exit function with atexit");
    cfg_if::cfg_if!{
    if #[cfg(target_has_atomic)]{
        use core::sync::atomic::{Ordering,AtomicBool};
        static ISEXITING: AtomicBool =AtomicBool::new(false);

        match ISEXITING.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed){
                // @TODO: do atexit functions
                Ok(true) => todo!(),
                Err(false) => todo!(),
                _ => unreachable!("Only true can be ok and false be err")
        }
        
    }
    }
}
/// Meta info for rslibc allowed to be parsed by objdump
// Look at build script for what is being generated
#[allow(warnings, reason = "Auto generated")]
mod meta{
    include!(concat!(env!("OUT_DIR"),"/meta.rs"));
}

