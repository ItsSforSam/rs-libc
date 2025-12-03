use crate::prelude::*;

pub fn munmap(addr: *mut c_void,len:usize)->Result<(),crate::errno::Errno>{
    todo!();
    if unsafe {syscall!(sys::SYS_munmap)} == -1{
        Errno::try_from(ERRNO.get() as u32).unwrap();
    }
    Ok(())
}
