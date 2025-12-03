use crate::prelude::*;

pub fn munmap(addr: *mut c_void,len:usize)->Result<(),crate::errno::Errno>{
    
    if unsafe {syscall!(api_sys::SYS_munmap, addr, len)} == -1{
        Errno::try_from(ERRNO.get() as u32).unwrap();
    }
    Ok(())
}
