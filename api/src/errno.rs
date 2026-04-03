#[doc(inline)]
pub use syscall::errno::*;
use core::ffi::c_int;


unsafe extern "C" {
    safe fn __get_errno_ptr() -> *mut c_int;
}

#[doc(hidden)]
pub trait MapErr<T>{

    fn set_errno(self) -> Option<T>;
}

impl<T> MapErr<T> for Result<T,Errno>{
    /// Sets [`Errno`] to the [`ERRNO`] thread local variable
    /// and returns the Option of success value
    /// 
    /// If [`None`] then the [`Errno`] was set, 
    /// the [`Some`] varient contains the same value as [`Ok`]  
    fn set_errno(self) -> Option<T> {
        match self{
            Ok(v) => Some(v),
            Err(v) => {
                // SAFETY: This function guarantees that it's returns to the thread local value (or a special value in single threaded apps)
                unsafe {*__get_errno_ptr() = v as c_int;}
                None
            }

        }
    }
}