//! Runtime routines for the runtime.
//! 
//! <b>These are internal details and should not be relied upon</b>




type MainFn = extern "C" fn(argc:core::ffi::c_int, argv: *mut *mut core::ffi::c_char)-> core::ffi::c_int;
/// Entrypoint from crt0. Should not be 
/// 
// @NOTE: If you modify the signature. It's a breaking change. (Once released)
//        This makes things...hard to update without breaking ABI
// SAFETY: prefixed to avoid collisions
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn __rslibc_start_entrypoint_1(
    mainfn:&MainFn,
    argv:*mut *mut core::ffi::c_char,
    argc:core::ffi::c_int,
    envp:*mut *mut core::ffi::c_char
)->!{
    // @TODO: initialize the c runtime
    api::quick_exit(mainfn(argc,argv))
}

