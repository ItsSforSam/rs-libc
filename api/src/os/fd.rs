//! Represents a owned and borrowed UNIX-like file descriptors

use core::{ffi::c_void, marker::PhantomData};
use crate::errno::MapErr as _;
use crate::syscall;

/// Raw file descriptor
// If hermit or motor is supported, then just use a i32
pub type RawFd = core::ffi::c_int;

// https://doc.rust-lang.org/stable/src/std/os/fd/owned.rs.html#26
// supports the niche of not being a negative one (an error)
type ValidRawFd = core::num::niche_types::NotAllOnes<RawFd>;

/// An owned file descriptor
/// 
/// Closes the file on dropped
#[repr(transparent)]
// #[rustc_nonnull_optimization_guaranteed]
#[derive(Debug)]
pub struct OwnedFd{
    fd:ValidRawFd
}

/// A borrowed file descriptor
#[derive(Debug)]
pub struct BorrowedFd<'s>{
    fd:ValidRawFd,
    _marker:PhantomData<&'s OwnedFd>
}

impl OwnedFd{
    /// # Safety
    /// Same safety guarantees as [`FromRawFd`] but cannot have the raw fd be `-1`
    pub const unsafe fn from_raw_fd_unchecked(fd:RawFd) -> Self{
        // SAFETY: caller guarantees
        OwnedFd { fd: unsafe {ValidRawFd::new(fd).unwrap_unchecked()} }
    }
    /// A const variant of the [`FromRawFd::from_raw_fd`]
    /// # SAFETY
    /// Same safety requirements as [`FromRawFd::from_raw_fd`]
    #[doc(hidden)] // this is a wrapper to allow const context
    pub const unsafe fn from_raw_fd(fd:RawFd) ->Self{
        OwnedFd { fd: ValidRawFd::new(fd).expect("fd != -1") }
    }
    /// Reads from fd and writes what is 
    pub fn read(&self,buf:&mut [u8])->crate::Result<usize>{
        // " if count is greater than SSIZE_MAX, the result is implementation-defined"
        read(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, core::cmp::min(isize::MAX as usize,buf.len()))
    }
    pub fn write(&self,buf:&[u8])->crate::Result<usize>{
        let ret = crate::io::stdio::write(self.fd.as_inner(), buf.as_ptr() as *const core::ffi::c_void, core::cmp::min(buf.len(),isize::MAX as usize))?;
        Ok(ret as usize)
    }
    pub fn flush(&self) -> crate::Result<()>{
        fsync(self.fd.as_inner())
    }
}

impl crate::io::Write for OwnedFd{
    fn write(&mut self,buf: &[u8]) -> crate::Result<usize> {
        Self::write(self,buf)
    }
    fn flush(&mut self) -> crate::Result<()> {
        Self::flush(self)
    }
}
impl crate::io::Read for OwnedFd {
    fn read(&mut self, buf:&mut [u8])->crate::Result<usize> {
        Self::read(self, buf)
    }
}
// It being const DOES NOT force 
pub const trait AsRawFd{

    fn as_raw_fd(&self) -> RawFd;
}
/// Allows you to construct an object from a file descriptor
pub trait FromRawFd{
    /// # Safety
    /// The fd passed must be a [owned file descriptor][io-safety] and should be open
    /// 
    /// [io-safety]:https://doc.rust-lang.org/std/io/index.html#io-safety
    unsafe fn from_raw_fd(fd:RawFd) -> Self;
}
impl FromRawFd for OwnedFd{
    
    unsafe fn from_raw_fd(fd:RawFd) -> Self {
       // SAFETY: caller guarantees safety has been met
       unsafe { OwnedFd::from_raw_fd(fd)}
    }
}

impl const AsRawFd for BorrowedFd<'_>{
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_inner()
    }
}

impl const AsRawFd for OwnedFd{
    fn as_raw_fd(&self) -> RawFd{
        self.fd.as_inner()
    }
}

impl Drop for OwnedFd{
    fn drop(&mut self) {
        // SAFETY: Owned fd should mean this is the only reference to the fd
        _ = unsafe {crate::io::stdio::close_fd(self.fd.as_inner())}
    }
}
/// Sync data to disk, won't sync file metadata
/// 
/// Look at [fsync(2)] for more details
/// 
/// [fsync(2)]: https://man.archlinux.org/man/fsync.2.en
// @TODO: better docs
pub fn fsync(fd:RawFd)->crate::Result<()>{
    // SAFETY: correct prams and types passed
    unsafe {syscall!(SYS_fsync,fd)?;}
    Ok(())
}
pub fn read(fd:RawFd,buf:*mut c_void,count:usize)-> crate::Result<usize>{
    // SAFETY: correct prams and typed
    let ret = unsafe {syscall!(
        SYS_read,
        fd,
        buf,
        count
    )}?;
    Ok(ret as usize)
}
#[unsafe(export_name = "read")]
extern "C" fn _read_export(fd:RawFd,buf:*mut c_void,count:usize) -> isize{
    match read(fd,buf,count).set_errno(){
        Some(v) => v as isize,
        None => -1
    }
}