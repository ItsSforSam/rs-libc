use core::ffi;


#[export_name = "__start_c_rt_v1"]
#[inline(never)]
/// A platform independent 
pub unsafe extern "C-unwind" fn start_c_rt(
    main_fn: unsafe extern "C" fn(argc:isize, *mut *mut argv, *mut *mut envp) -> ffi::c_int
) -> ffi::c_int{


}