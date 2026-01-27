use crate::prelude::*;
use bitflags::bitflags;
pub fn munmap(addr: *mut c_void,len:usize)->Result<(),crate::errno::Errno>{
    //SAFETY: calls with proper parameters of correct type
    unsafe {crate::syscall!(SYS_munmap, addr, len)}?;
    Ok(())
}

bitflags! {
    pub struct MMapProt: c_uint{
        const Exec  = sys::PROT_EXEC;
        const Read  = sys::PROT_READ;
        const Write = sys::PROT_WRITE;
        const None  = sys::PROT_NONE;
    }
}

bitflags! {
    pub struct MMapFlags: u32{
        const Shared         = sys::MAP_SHARED;
        const SharedValidate = sys::MAP_SHARED_VALIDATE;
        const Private        = sys::MAP_PRIVATE;
        const Bit32          = sys::MAP_32BIT;
        const Anonymous      = sys::MAP_ANONYMOUS;
        // Ignored as caused DDoS
        const DenyWrite      = sys::MAP_DENYWRITE;
        // Ignored
        const Executable     = sys::MAP_EXECUTABLE;

        const Fixed          = sys::MAP_FIXED;
        const FixedNoReplace = sys::MAP_FIXED_NOREPLACE;
        const GrowsDown      = sys::MAP_GROWSDOWN;
        const HugetLB        = sys::MAP_HUGETLB;
        // const HugetLB_2MP    = sys::MAP_HUGE_2MB;
        // const HugetLB_1GB    = sys::MAP_HUGE_1GB;

        const Locked         = sys::MAP_LOCKED;
        const NonBlock       = sys::MAP_NONBLOCK;
        const NoReserve      = sys::MAP_NORESERVE;
        const Stack          = sys::MAP_STACK;
        const Sync           = sys::MAP_SYNC;
        // const Uninitialized =   sys::MAP_UNINITIALIZED;
        // Have all bytes be "known" as more flags can be introduced
        //
        const _ = !0;
    }
}


pub fn mmap(
    addr:*mut c_void,
    size: usize, // size_t
    proto:MMapProt,
    flags: MMapFlags,
    fd: c_int,
    offset:c_long
)->crate::Result<*mut c_void>{
    
    //SAFETY: Puts the proper syscall with the proper parameters
    let v = unsafe {crate::syscall!(__mmap2_sys,
                    addr,
                    size,
                    proto.bits(),
                    flags.bits(),
                    fd,
                    offset
                )}?;
    Ok(v as *mut c_void)

}
