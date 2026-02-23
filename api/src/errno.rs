#[doc(inline)]
pub use syscall::errno::*;
use core::ffi::c_int;
use core::cell::Cell;

/// The errno of the current thread
#[thread_local]
pub static ERRNO:Cell<c_int> = Cell::new(0);

/// This is aliased to C as the errno with
/// 
/// ```C
/// #define errno {*__get_errno_ptr()}
/// ```
// SAFETY: double underscore prefix makes it impl specific and there shouldn't be and other libc loaded 
#[unsafe(no_mangle)]
extern "C" fn __get_errno_ptr() -> *mut c_int{
    ERRNO.as_ptr()  
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
                ERRNO.set(v as i32);
                None
            }

        }
    }
}