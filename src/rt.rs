//! Runtime routines for the runtime.
//! 
//! <b>These are internal details and should not be relied upon</b>
use rslibc_common::Libc;
pub static LIBC: Libc = Libc::default();




/// This is aliased to C as the errno with
/// 
/// ```C
/// #define errno {*__get_errno_ptr()}
/// ```
// SAFETY: double underscore prefix makes it impl specific and there shouldn't be and other libc loaded 
#[unsafe(no_mangle)]
#[linkage = "weak"] // TODO: find something better
extern "C" fn __get_errno_ptr() -> *mut core::ffi::c_int{
    LIBC.get_errno_raw()
}


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
    // @TODO: initialize the c runtime, like the libc struct
    // get the auxiliary vector if available
    api::quick_exit(mainfn(argc,argv))
}

