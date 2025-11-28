use core::ffi::{self, c_char};


#[unsafe(export_name = "__start_c_rt_v1")]
#[inline(never)]
/// A platform independent 
pub unsafe extern "C-unwind" fn start_c_rt(
    main_fn: unsafe extern "C" fn(argc:isize, argv:*mut *mut c_char, envp: *mut *mut c_char) -> ffi::c_int
) -> ffi::c_int{
    todo!()

}