use core::{marker::{PhantomData, PhantomPinned}, mem::ManuallyDrop, sync::atomic::AtomicBool};



// This is passed out thru the api
// BUT only as a pointer with the 
/// A Representation of a open file
/// 
/// This can be used with any file descriptor and not strictly a "file"
#[non_exhaustive]
// #[repr(C)]
#[derive(Debug)]
#[doc(alias = "FILE")]
#[doc(alias = "IO_FILE")]
#[cfg_attr(test, repr(C))]
pub struct File {
    // @NOTE: if more fields are added, edit close_explicitly to ensure no leaks occur 
    /// The internal file descriptor
    // This can never be -1
    fd: ManuallyDrop<api::os::fd::OwnedFd>,
    lock:AtomicBool,
    // mode:Mode,
    // Since this is used in C code, we shouldn't move it, maybe
    _marker:core::marker::PhantomData<PhantomPinned>
}
impl File{
    /// Constructs a `FILE` from a file descriptor
    /// 
    /// 
    /// # PANICS
    /// If fd is -1 which is never valid
    /// # SAFETY
    /// Does not check if a file descriptor is a valid fd. 
    /// This does not prevent the kernel from checking, and the <code>try_*</code> functions
    /// returning [`EBADF`]. The non try versions will assume these never occur and cause undefined behaver
    /// 
    /// [`EBADF`]: crate::errno::Errno::EBADF
    #[doc(alias = "fdopen")]
    pub const unsafe fn from_fd_unchecked(fd:crate::os::fd::RawFd)->Self{
        File {
            // SAFETY: caller guarantees this a valid file descriptor
            fd: unsafe { OwnedFd::from_raw_fd(fd)},
            lock:AtomicBool::new(false),
            _marker:PhantomData
            // mode,
            // _pin:core::marker::PhantomPinned
            // lock: AtomicBool::new(false)
            
        }
    }
    pub fn write(&self,buf:&[u8])->crate::Result<usize>{
        self.fd.write(buf)
    }
    pub fn read(&self,buf:& mut[u8])->crate::Result<usize>{
        self.fd.read(buf)
    }
    /// Gives back the file descriptor
    /// 
    /// Use of the getter is due to preventing accidental modification
    /// of the file descriptor, as it should not be changed
    pub const fn fd(&self)->RawFd{
        self.fd.as_raw_fd()
    }


    /// A fallible clone
    /// 
    /// Duplicates a file descriptor
    /// 
    /// Calls [dup(2)]
    /// 
    /// # Errors
    /// 
    /// * [`EBADF`] - if the underlying fd is not a valid file descriptor. This should only occur if [`from_fd_unchecked`] was called with invalid
    ///   fd. This can safely be guaranteed to never occur if the unsafe function had it's safety guarantees upheld.
    /// * [`EMFILE`] - Per process limit was reached of open file descriptors
    /// * [`ENOMEM`] - Insufficient kernel memory is available
    /// 
    /// 
    /// [`EBADF`]: crate::errno::Errno::EBADF
    /// [`EMFILE`]: crate::errno::Errno::EMFILE
    /// [`ENOMEM`]: crate::errno::Errno::ENOMEM
    /// [`from_fd_unchecked`]: File::from_fd_unchecked
    pub fn try_clone(&self) ->crate::Result<Self>{

        todo!("Get dup definition")
    }
    /// Does the same as dropping which closes the file. And returns any errors
    /// 
    /// # Errors
    /// Returns a tuple of `(Self,Errno)` so you can still operate on the File after a error as 
    /// 
    ///  - EBADF  : fd is not a valid open file descriptor. This shouldn't occur in most cases
    /// - EINTR  : the close call was interrupted by a signal
    /// - EIO    : An I/O error occurred
    /// - ENOSPEC
    /// - EDQUOT : On NFS this will not occur until writes that occur after the quota exceeds
    /// 
    pub fn close_explicitly(self)->Result<(),(Self,Errno)>{
        // This makes sure we don't drop the value twice, which can cause undefined behaver
        let f = core::mem::ManuallyDrop::new(self);
        // SAFETY: we own the file descriptor
        match unsafe {close_fd((&f).fd())}{
            Ok(_) => {
            
                Ok(())
            },
            Err(e) => Err((core::mem::ManuallyDrop::into_inner(f), e))
        }
    }
    
}

impl api::io::Read for File{
    fn read(&mut self, buf:&mut [u8])->api::Result<usize> {
        self.fd.read(buf)
    }
}
impl crate::io::Write for File{
    fn write(&mut self,buf: &[u8]) -> api::Result<usize> {
        self.fd.write(buf)
    }

    fn flush(&mut self) -> api::Result<()> {
        self.fd.flush()
    }
}

impl Clone for File{
    /// Clones a file object
    fn clone(&self) -> Self {
        
        match self.try_clone(){
            Ok(v) => v,
            Err(v) =>{
                use api::errno::Errno::*;
                match v{
                    // SAFETY: Should always be valid
                    EBADE => unsafe{core::hint::unreachable_unchecked()},
                    ENOMEM => alloc::alloc::handle_alloc_error(Default::default()),
                    // SAFETY: No other errnos can be returned as it runs dup underneath
                    // https://man.archlinux.org/man/dup2.2.en#ERRORS
                    _ => unsafe{core::hint::unreachable_unchecked()}

                }
            }
        }
    }
}
