//! Holds environment information
#[cfg(target_has_atomic="ptr")]
use core::sync::atomic::{AtomicIsize,AtomicPtr,Ordering,Atomic};

static ARGC:AtomicIsize = AtomicIsize::new(0);
static ARGV:Atomic<*mut *const u8> = AtomicPtr::new(core::ptr::null_mut());

// static ARGV_ARRAY: extern "C" fn(core::ffi::c_int)

