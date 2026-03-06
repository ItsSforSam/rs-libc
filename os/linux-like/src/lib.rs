//! Linux specific syscalls and functionality
#![no_std]

use syscall::{syscall,errno::Errno};

unsafe extern "C" {
static mut end:*mut core::ffi::c_char; // current page break 
}

/// Change the data segment size
#[expect(clippy::multiple_unsafe_ops_per_block, reason = "We address both in unsafe comments")]
pub fn brk(addr: *mut core::ffi::c_void)->Result<*mut core::ffi::c_void, Errno>{
    // SAFETY: valid parameters
    let ret:*mut core::ffi::c_void = addr.with_addr(unsafe {
        syscall!(SYS_brk, addr)
        
            // SAFETY: it's listed that in linux that this syscall will just return the current page break
            //          and not follow normal error-ing
        .unwrap_unchecked() as usize
    });
    // SAFETY: The end user should not rely upon the value
    // and this value will fix it self anyway if a race did occur
    unsafe {end = ret.cast()};
    if ret < addr{
        return Err(Errno::ENOMEM);
    }
    
    return Ok(ret);

}