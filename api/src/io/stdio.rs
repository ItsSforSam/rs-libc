//! Allows for standard IO
use core::{ffi::CStr, fmt, mem::MaybeUninit};

use alloc::boxed::Box;
use bitflags::bitflags;

use crate::{os::fd::RawFd, prelude::*, syscall};





pub fn write(
    fd:c_int,
    buffer:*const c_void,
    size: usize

)->crate::Result<usize>{ // size_t -1 is for errors which is why it's ssize_t
    // SAFETY: Calls the syscall with the proper values
    let v = unsafe{
        syscall!(SYS_write,
        fd,
        buffer,
         size)?
        };
        
        Ok(v as usize)
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
/// Represents the [`EINVAL`] for when the mode provided was invalid
/// 
/// 
/// [`EINVAL`]: variant@syscall::errno::Errno::EINVAL
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub struct InvalidModeStr(Box<str>);

impl From<InvalidModeStr> for syscall::errno::Errno{

    fn from(_: InvalidModeStr) -> Self {
        syscall::errno::Errno::EINVAL
    }
}
impl fmt::Display for InvalidModeStr{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"Invalid arguments for mode string as {}",self.0)
    }
}
impl core::error::Error for InvalidModeStr {}
impl core::str::FromStr for Mode{
    type Err = InvalidModeStr;
    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        // We are assuming that Box never fails, which, due to the small values, and you shouldn't
        // be taking user input with your mode string (and if you are, you're stupid)
        if !s.is_ascii(){ // all valid modes are ascii
            return Err(InvalidModeStr(s.into()));
        }
        match s {
            "r" => Ok(Mode::Read),
            "w" => Ok(Mode::Write | Mode::Create | Mode::Truncate),
            "a" => Ok(Mode::Write | Mode::Create | Mode::Append),
            "r+"=> Ok(Mode::ReadWrite),
            "w+"=>Ok(Mode::ReadWrite | Mode::Create | Mode::Truncate),
            "a+"=>Ok(Mode::Write | Mode::Create | Mode::Append),
            invalid => Err(InvalidModeStr(invalid.into()))
        }

        
    }
}
/// Same as [close(2)]
/// 
/// # Safety
/// If a fd is currently owned or this is used again, can cause undefended behavior
pub unsafe fn close_fd(fd: crate::os::fd::RawFd) -> crate::Result<()>{
    // SAFETY: valid prams
    unsafe {syscall!(SYS_close,fd)}?;
    Ok(())
}

bitflags! {
    #[repr(transparent)]
    pub struct StatFlags: c_int{
        const EmptyPath = sys::AT_EMPTY_PATH as c_int;
        const NoAutoMount = sys::AT_NO_AUTOMOUNT as c_int;
        const NoFollowSymink = sys::AT_SYMLINK_NOFOLLOW as c_int;
        #[doc(hidden)]
        const _ = !0;
    }
}

/// Calls to [stat(2)]
/// 
/// [stat(2)]: https://man.archlinux.org/man/stat.2.en
pub fn stat(path: &CStr, statbuf:&mut MaybeUninit<api_sys::stat>) -> crate::Result<()>{
    // SAFETY: We use valid prams
    unsafe {syscall!(SYS_newfstatat, sys::AT_FDCWD, path.as_ptr(),statbuf.as_mut_ptr(),0)?};
    Ok(())
}

pub fn fstat(fd:RawFd,  statbuf:&mut MaybeUninit<api_sys::stat> ) -> crate::Result<()>{
    // SAFETY: We use valid prams
    unsafe {syscall!(SYS_newfstatat, c"".as_ptr(),statbuf.as_mut_ptr(),StatFlags::EmptyPath.bits())}?;
    Ok(())
}

pub fn fstatat(dirfd:RawFd,path: &CStr,statbuf:&mut MaybeUninit<api_sys::stat>,flags:StatFlags) -> crate::Result<()>{
    // SAFETY: We use valid prams
    unsafe {syscall!(SYS_newfstatat, path.as_ptr(),statbuf.as_mut_ptr(),flags.bits())}?;
    Ok(())
}