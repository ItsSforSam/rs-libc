//! Allows for standard IO
use core::sync::atomic::{AtomicBool, Ordering};

use bitflags::bitflags;

use crate::{os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd}, prelude::*, syscall};




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
pub struct File {
    /// The internal file descriptor
    // This can never be -1
    fd: crate::os::fd::OwnedFd,

    // mode:Mode,
    // Since this is used in C code, we shouldn't move it, maybe
    // _pin:core::marker::PhantomPinned
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
    pub const unsafe fn from_fd_unchecked(fd:crate::os::fd::RawFd,mode:Mode)->Self{
        File {
            // SAFETY: caller guarantees this a valid file descriptor
            fd: unsafe { OwnedFd::from_raw_fd(fd)},
            // mode,
            // _pin:core::marker::PhantomPinned
            // lock: AtomicBool::new(false)
            
        }
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
    pub fn try_clone(&self) ->crate::Result<Self>{

        todo!("Get dup definition")
    }
    /// Does the same as dropping which closes the file
    /// 
    /// # Errors
    //  - EBADF  : fd is not a valid open file descriptor. This shouldn't occur in most cases
    /// - EINTR  : the close call was interrupted by a signal
    /// - EIO    : An I/O error occurred
    /// - ENOSPEC
    /// - EDQUOT : On NFS this will not occur until writes that occur after the quota exceeds
    /// 
    /// # SAFETY
    /// Do not use this object if returns [`Ok`] as this uses a reference to ensure you have access
    /// if it fails
    pub unsafe fn close(&mut self)->crate::Result<()>{
        // SAFETY: we own the file descriptor
        unsafe {close_fd(self.fd())}?;
        Ok(())
    }
    
}

impl Clone for File{
    /// Clones a file object
    fn clone(&self) -> Self {
        
        match self.try_clone(){
            Ok(v) => v,
            Err(v) =>{
                use crate::errno::Errno::*;
                match v{
                    // SAFETY: Should always be valid
                    EBADE => unsafe{core::hint::unreachable_unchecked()},
                    #[cfg(feature = "alloc")]
                    ENOMEM => alloc::alloc::handle_alloc_error(core::alloc::Layout::for_value(self)),
                    #[cfg(not(feature = "alloc"))]
                    ENOMEM => panic!("Kernel cannot allocate for new file"),
                    // SAFETY: No other errnos can be returned as it runs dup underneath
                    // https://man.archlinux.org/man/dup2.2.en#ERRORS
                    _ => unsafe{core::hint::unreachable_unchecked()}

                }
            }
        }
    }
}

impl Drop for File{
    fn drop(&mut self) {
        // SAFETY: we own the file descriptor
        // Here we just ignore
        unsafe {_ = self.close()}
        
    }
}

fn write(
    fd:c_int,
    buffer:*const c_void,
    size: usize

)->crate::Result<isize>{ // ssize_t
    // SAFETY: Calls the syscall with the proper values
    let v = unsafe{
        syscall!(api_sys::SYS_write,
        fd,
        buffer,
         size)?
        };
        
        Ok(v as isize)
    }
bitflags! {
    /// File mode options
    /// 
    /// The values of [`Read`], [`Write`] and [`ReadWrite`] 
    /// are required to be set to correctly open a file
    /// 
    #[derive(Debug)]
    pub struct Mode:core::ffi::c_uint{
        // For somereason the sys crate isn't getting the values
        // maybe a bug with the headers using octo.
        // Just inline these for now.
        // values are in your 
        // `/usr/asm-generic/fcntl.h`
        // but c
        /// Opens for reading only
        const Read = sys::O_RDONLY;
        /// Opens for write only
        const Write = sys::O_WRONLY;
        /// Open a File for reading and writing
        const ReadWrite = sys::O_RDWR;
        /// Sets file offset to the end of the file
        const Append = sys::O_APPEND;
        /// Enable signal-driven IO
        #[doc(alias = "O_ASYNC")]
        const Async = sys::FASYNC;
        /// Enables close on exec on a file descriptor
        /// 
        /// This allows multithreaded programs to enable it
        /// without a race condition with [fcntl(2)] being called at
        /// the same time as [fork(2)] plus [execve(2)]
        /// 
        /// Available since Linux 2.6.23
        /// 
        /// [fcntl(2)]: https://man.archlinux.org/man/fcntl.2.en
        /// [fork(2)]: https://man.archlinux.org/man/fork.2.en
        /// [execve(2)]:https://man.archlinux.org/man/execve.2.en
        const CloseOnExec = sys::O_CLOEXEC;
        /// Create if file does not exist
        const Create =  sys::O_CREAT;
        /// Minimize cache effects on IO
        /// 
        /// This will degrade performance in most cases, but useful for apps
        /// implementing their own caching
        /// 
        /// Provides functionality similer to the (now depreciated) interface for
        /// block interfaces in [raw(8)]
        /// 
        /// Available since Linux 2.4.10
        /// 
        // Arch doesn't have this man page
        /// [raw(8)]:https://linux.die.net/man/8/raw
        const Direct = sys::O_DIRECT;
        /// Cause [`open`] to fail if path is not a directory
        // const Directory = sys::O_DIRECTORY;
        /// Used in conjuction with [`Create`] and will fail with [`EEXIST`]
        /// if path exists
        /// 
        /// [`EEXIST`]:crate::errno::Errno::EEXIST`
        const ForceCreate = sys::O_EXCL;
        /// Allows files which cannot fit in `off_t`
        /// 
        /// In C, the ` _LARGEFILE64_SOURCE` macro must be defined before any include
        /// 
        const O_LARGEFILE = sys::O_LARGEFILE;
        /// Don't update last read access
        /// 
        /// This will only work if the following is true
        /// * The effective uid of the process is the same as the owner uid of the file
        /// * The process has the `CAP_FOWNER` capability
        /// 
        /// Used by backup services 
        /// 
        /// Available since Linux 2.6.8
        const NoAccess   = sys::O_NOATIME;
        /// If path refers to a terminal. It will *not* become process's
        /// controlling terminal. Even if the process does not have one.
        /// 
        /// See [tty(4)][https://man.archlinux.org/man/tty.4.en]
        const NoTTY      = sys::O_NOCTTY;
        /// If the basename of the path is a symbolic link then fail with [`ELOOOP`] 
        /// 
        /// [`ELOOOP`]:crate:errno::Errno::ELOOP
        const NoFollow   = sys::O_NOFOLLOW;
        /// Try to not block the process
        /// 
        /// Currently in Linux has no effect on regular files and block devices.
        /// 
        /// See [fifo(7)] on handling on named pipes. <br>
        /// See [fcntl(2)] for info on file locks.
        /// 
        /// [fifo(7)]: https://man.archlinux.org/man/fifo.7.en
        /// [fcntl(2)]:https://man.archlinux.org/man/fcntl.2.en
        #[doc(alias = "O_NDELAY")]
        const NoBlock    = sys::O_NONBLOCK;
        /// Allows to perform acts at only the file descriptor level.
        ///
        /// 
        /// Will cause functions which perform actions on file to fail with [`EBADF`] 
        /// The following can be used with the resulting file descriptor
        ///
        ///     * [close(2)].
        ///     * [fchdir(2)], if the file descriptor refers to a directory (since Linux 3.5).
        ///     * [fstat(2)] (since Linux 3.6).
        ///     * [fstatfs(2)] (since Linux 3.12).
        ///     * Duplicating the file descriptor (dup(2), fcntl(2) F_DUPFD, etc.).
        ///     * Getting and setting file descriptor flags (fcntl(2) F_GETFD and F_SETFD).
        ///     * Retrieving open file status flags using the fcntl(2) F_GETFL operation: the returned flags will include the bit O_PATH.
        ///     * Passing the file descriptor as the dirfd argument of openat() and the other "*at()" system calls. This includes linkat(2) with AT_EMPTY_PATH (or via procfs using AT_SYMLINK_FOLLOW) even if the file is not a directory.
        ///     * Passing the file descriptor to another process via a UNIX domain socket (see SCM_RIGHTS in unix(7)).
        /// 
        /// Available since Linux 2.6.39
        /// 
        // @TODO: add links to man page
        /// [`EBADF`]: api::errno::Errno::EBADF
        /// [close(2)]
        /// [fchdir(2)]
        /// [fstat(2)]
        /// [fstatfs(2)]

        const NoOpen     = sys::O_PATH;
        /// Sync data to the hardware
        const Sync       = sys::O_SYNC;

        /// Create a unnamed temporary inode into the file system.
        /// 
        /// All content will be lost when all file descriptors are closed.
        /// 
        /// if [`ForceCreate`] is not specified then [linkat(2)] can be used
        /// to add it to be permanently to the file system
        ///
        ///  Available since Linux 3.11
        /// 
        /// [linkat(2)]:https://man.archlinux.org/man/linkat.2.en
        const Temp       = sys::O_TMPFILE;
        /// If (normal) file exists, and allows for writing truncate length to 0.
        /// If the file is fifo or terminal, then this is ignored
        /// 
        /// Otherwise it's "unspecified"
        const Truncate  = sys::O_TRUNC;
        /// All known bits
        const _ = !0;
    }
}


// SAFETY: 0 is always stdin
pub static STDIN:File = unsafe {File::from_fd_unchecked(0, Mode::Read)};
// SAFETY: 1 is always stdout
pub static STDOUT:File = unsafe { File::from_fd_unchecked(1, Mode::Write)};
// SAFETY: 2 is always stderr
pub static STDERR:File = unsafe { File::from_fd_unchecked(1, Mode::Write)};


/// Same as [close(2)]
/// 
/// # Safety
/// If a fd is owned or used again, can cause undefended behavior
pub(crate) unsafe fn close_fd(fd: crate::os::fd::RawFd) -> crate::Result<()>{
    // SAFETY: valid prams
    unsafe {syscall!(sys::SYS_close,fd)}?;
    Ok(())
}

macro_rules! define_outs {
    (
        $( $id:ident($fd:literal) = $mode:ident ),*) => {
        $(
        #[repr(transparent)]
        pub struct $id{
            inner:File

        }
        impl $id{
            const fn new()->Self{
                $id {
                    // Safety: used for stdout, stderr,stdin, which aren't actual files
                    inner: unsafe { File::from_fd_unchecked($fd, Mode::$mode)}
                }
            }
        }
        )*
    };
}
define_outs!(
    Stdin(0)=Read,
    Stdout(1)=Write,
    Stderr(2)=Write
);



