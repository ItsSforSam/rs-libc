//! Represents a owned and borrowed UNIX-like file descriptors

/// Raw file descriptor
// If hermit or motor is supported, then just use a i32
pub type RawFd = core::ffi::c_int;

// https://doc.rust-lang.org/stable/src/std/os/fd/owned.rs.html#26
// supports the niche of not being a negative one (an error)
type ValidRawFd = core::num::niche_types::NotAllOnes<RawFd>;

#[repr(transparent)]
#[derive(Debug)]
pub struct OwnedFd{
    pub(crate) fd:ValidRawFd
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

pub trait AsRawFd{

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
impl AsRawFd for OwnedFd{
    fn as_raw_fd(&self) -> RawFd{
        self.fd.as_inner()
    }
}