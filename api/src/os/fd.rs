//! Represents a owned and borrowed UNIX-like file descriptors

use core::marker::PhantomData;

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