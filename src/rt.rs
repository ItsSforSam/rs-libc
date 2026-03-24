//! Runtime routines for the runtime.
//! 
//! <b>These are internal details and should not be relied upon</b>
use core::ffi::*;

use api::{quick_exit, sys::SIGABRT};
use rslibc_common::Libc;
pub static LIBC: Libc = Libc::default();


//CSpell: words mainfn

/// This is aliased to C as the errno with
/// 
/// ```C
/// #define errno {*__get_errno_ptr()}
/// ```
// SAFETY: double underscore prefix makes it impl specific and there shouldn't be and other libc loaded 
#[unsafe(no_mangle)]
#[linkage = "weak"] // TODO: find something better
extern "C" fn __get_errno_ptr() -> *mut c_int{
    LIBC.get_errno_raw()
}


type MainFn = extern "C" fn(argc:c_int, argv: *mut *mut c_char)-> c_int;
/// Entrypoint from crt0.
/// 
// @NOTE: If you modify the signature. It's a breaking change. (Once released)
//        This makes things...hard to update without breaking ABI
// SAFETY: prefixed to avoid collisions
#[unsafe(no_mangle)]
pub unsafe extern "C-unwind" fn __rslibc_start_entrypoint_1(
    mainfn:*const MainFn,
    argv:*mut *mut c_char,
    argc:c_int,
    envp:*mut *mut c_char
)->!{
    // @TODO set up environment variables
    let _ = envp;
    // @TODO: initialize the c runtime, like the libc struct
    // get the auxiliary vector if available
    //safety: 
    api::quick_exit(unsafe {(*mainfn)(argc,argv)})
}
/// A way to start up libc's runtime
/// 
/// This is an alternative to [`__rslibc_start_entrypoint_1`] based on the standard defined in [LSB Spec]
/// 
/// 
/// 
/// # SAFETY
/// 
/// * `mainfn` must be nonnull
/// * If `argc` is non-zero, then `unbound_argv` must be point to the proper memory, not null or dangling
/// * `init_fn`, `finish_fn`, `resource_unload` *can* be null, but must be a valid function pointer when non-null.
///   If Null it's effectively no-op for the given parameter
/// * `stack_end` should point to the end of the stack, and be non-null
/// 
/// [LSB Spec]: https://refspecs.linuxbase.org/LSB_3.1.0/LSB-generic/LSB-generic/baselib---libc-start-main-.html
#[unsafe(no_mangle)]
#[expect(unused_variables, reason ="The parameters are pained to be used, but don't want it to effect the signature")]
pub unsafe extern "C-unwind" fn __libc_start_main(
    mainfn:*const MainFn,
    argc:c_int,
    unbound_argv: *mut *mut c_char, 
    init_fn: *const extern "C" fn(),
    finish_fn: *const extern "C" fn(),
    resource_unload: *const extern "C" fn(),
    stack_end: *mut c_void

)->c_int{
    if core::hint::unlikely(mainfn.is_null()){

        let _ = api::raise(SIGABRT as _);
        
        // SAFETY: No signal handler should be registered and the caller already
        // violated safety of the function
        unsafe {core::hint::unreachable_unchecked()}
    }
    //@TODO
    // SAFETY: caller guarantees say must be valid pointer
    let main = unsafe {*mainfn};
    let exit_code = main(argc,unbound_argv);
    quick_exit(exit_code);
}

// #[link(name = "threadlib")]
unsafe extern "C"{
    

    #[doc = include_str!("../meta/docs/abi-breakage.md")]
    /// 
    /// # SAFETY
    /// 
    /// If [`Some()`] then this should only be called once
    /// 
    /// 
    #[expect(improper_ctypes, reason ="We are interfacing with Rust-only")]
    #[linkage = "extern_weak"]
    static __rsinit_thread_lib:Option<extern "C" fn(panic_handler:extern "Rust" fn(&core::panic::PanicInfo)->!)->u8>;
}