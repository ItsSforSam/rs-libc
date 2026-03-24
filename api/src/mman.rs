use crate::prelude::*;
use bitflags::bitflags;
/// # SAFETY
/// This will have mapped memory pointed at the the pointer with the length of len.
/// If any pointers are used after this call it may cause a segfault, or you may get "lucky"
/// and the memory gets mapped again so the addr is now valid, which can corrupt memory
pub unsafe fn munmap(addr: *mut c_void,len:usize)->Result<()>{
    //SAFETY: calls with proper parameters of correct type
    unsafe {crate::syscall!(SYS_munmap, addr, len)}?;
    Ok(())
}

bitflags! {
    /// Gives memory certain protections
    #[repr(transparent)]
    pub struct MMapProt: c_uint{
        /// Pages may be executed.
        const Exec  = sys::PROT_EXEC;
        /// Pages may be read.
        const Read  = sys::PROT_READ;
        /// Pages may be written too.
        const Write = sys::PROT_WRITE;
        /// Page may not be accessed
        const None  = sys::PROT_NONE;
    }
}

bitflags! {
    /// Flags used for [`mmap`].
    /// 
    /// The only required flags are using either [`Private`] or [`Shared`]
    /// 
    /// 
    /// Read the [mmap(2)] man page for more info
    /// 
    /// 
    /// [mmap(2)]: https://man.archlinux.org/man/mmap.2.en
    /// [`Private`]:MMapFlags::Private
    /// [`Shared`]:MMapFlags::Shared
    
    pub struct MMapFlags: c_int{
        /// Modifications are public to other processes.
        /// 
        /// To precisely control when updates are carried out, check out [msync(2)]
        /// 
        /// [msync(2)]: https://man.archlinux.org/man/msync.2.en
        const Shared         = sys::MAP_SHARED as _;
        /// Same as [`Shared`] but in the case of any unknown flags
        /// 
        /// [`Shared`]: MMapFlags::Shared
        const SharedValidate = sys::MAP_SHARED_VALIDATE as _;
        /// Modifications are public
        const Private        = sys::MAP_PRIVATE as _;
        ///Put the mapping into the first 2 Gigabytes of the process address space. This flag is supported only on x86-64, for 64-bit programs
        /// 
        /// This is not required on modern system
        #[cfg(all(target_arch = "x86_64",target_os = "linux"))]
        const Bit32          = sys::MAP_32BIT as _;
        /// Don't have a mapping be associated with any file
        const Anonymous      = sys::MAP_ANONYMOUS as _;
        // Ignored as caused DDoS
        /// Have writes to file fail with  ETXTBSY. This is now ignored to prevent DDoS attacks
        const DenyWrite      = sys::MAP_DENYWRITE as _;
        // Ignored
        /// This flag is ignored
        const Executable     = sys::MAP_EXECUTABLE as _;
        /// Have address requested be fixed and not taken as a hint
        /// 
        /// This may overwrite data. If possible use [`FixedNoReplace`] if possible
        /// 
        /// [`FixedNoReplace`]: MMapFlags::FixedNoReplace
        const Fixed          = sys::MAP_FIXED as _;
        #[cfg(target_os="linux")]
        /// Same as [`Fixed`] but will fail if it is overlapping with already mapped memory
        const FixedNoReplace = sys::MAP_FIXED_NOREPLACE as _;
        #[cfg(target_os="linux")]
        /// This flag is used for stacks. Used to indicate that the memory should grow down in memory
        const GrowsDown      = sys::MAP_GROWSDOWN as _;
        // const HugetLB        = sys::MAP_HUGETLB as _;
        // const HugetLB_2MP    = sys::MAP_HUGE_2MB;
        // const HugetLB_1GB    = sys::MAP_HUGE_1GB;
        /// Mark mapped regions as if was used with mlock,
        /// 
        /// Have mmap not fail with ENOMEM, but may have faults happen latter
        const Locked         = sys::MAP_LOCKED as _;
        /// Only usfull if Don't perform read-ahead.
        const NonBlock       = sys::MAP_NONBLOCK as _;
        /// Don's reserve swap space for this mapping. 
        /// This may cause a SIGSEGV be raised if no physical memory is aviable
        const NoReserve      = sys::MAP_NORESERVE as _;
        /// Allocate a mapping suitable for a process or thread stack
        /// 
        /// Currently no-op on linux, but is valid on the BSDs
        const Stack          = sys::MAP_STACK as _;
        /// 
        const Sync           = sys::MAP_SYNC as _;
        // const Uninitialized =   sys::MAP_UNINITIALIZED;
        // Have all bytes be "known" as more flags can be introduced
        //
        #[doc(hidden)]
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
)->Result<*mut c_void>{
    
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
