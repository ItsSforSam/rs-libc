use core::{cell::UnsafeCell, ffi::CStr, marker::{PhantomData, PhantomPinned}, mem::{ManuallyDrop, MaybeUninit}, ptr::NonNull, sync::atomic::AtomicBool};

use api::{os::fd::{AsRawFd,OwnedFd,RawFd}, sys::pid_t, threading::get_tid};
use api::prelude::*;
pub use api::io::stdio::*;
use core::sync::atomic::{Atomic,Ordering};


/// A special array the file struct uses
/// 
/// The exact contents are an implication detail can cannot be relied upon outside rslibc
/// 
/// This allows us to ensure we are looking
const FILE_SIG:[u8;4] = *b"FILE";

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
    header:[u8;4],
    // Any fields that can be written/read from without the lock, must be atomic

    // @NOTE: if more fields are added, edit close_explicitly to ensure no leaks occur 
    /// The internal file descriptor
    // This can never be -1
    fd: ManuallyDrop<api::os::fd::OwnedFd>,

    /// The internal lock
    lock:AtomicBool,
    /// The thread ID of the owner
    /// This helps prevent dead locks if for example stdout is tried to be retrieved
    /// from a signal handler
    /// 
    /// Access to this value may be [`Relaxed`] due to a lock being relaced
    /// 
    /// [`Relaxed`]: core::sync::atomic::Ordering
    owner_tid:Atomic<api::sys::pid_t>,
    // This doesn't have to be atomic due it only being observed in thread boundaries 
    lock_count:UnsafeCell<u32>,
    open: UnsafeCell<bool>,
    /// The backing buffer
    /// 
    /// If it's None, it stays null
    /// 
    /// This does not use the niche, but we are sure it won't be null
    buffer: Option<UnsafeCell<NonNull<[c_char]>>>,
    // mode:Mode,
    // Since this is used in C code, we shouldn't move it, maybe
    _marker:core::marker::PhantomData<PhantomPinned>
}
// SAFETY: We use proper locking
unsafe impl core::marker::Send for File  {}
// SAFETY: We use proper locking
unsafe impl core::marker::Sync for File {}
impl File{


    /// Open a file at `path`
    // cspell: words fopen
    #[doc(alias = "fopen")]
    pub fn open<P:AsRef<CStr>>(path:P, flags:api::io::Mode)->Result<File>{
        let p =path.as_ref();
        let fd_raw = api::io::open(p.as_ptr(), flags.bits() as c_int, 0)?;
        if fd_raw < 0{
            // We don't know what happened, and it should be caught by syscall macro
            return Err(Errno::EBADF);
        }
        let mut fd = ManuallyDrop::new(unsafe {api::os::fd::OwnedFd::from_raw_fd_unchecked(fd_raw)});

        let mut r = File{
            header:FILE_SIG,
            fd,
            lock: AtomicBool::new(false),
            owner_tid: Atomic::<api::sys::pid_t>::new(0),
            lock_count: UnsafeCell::new(0),
            // Assume no buffer, we are 
            buffer: None,
            open:UnsafeCell::new(true),
            _marker: PhantomData
        };

    

        let mut stats: MaybeUninit<api::sys::stat> = MaybeUninit::zeroed();
        let mut size:usize;
        match fstat(fd_raw,&mut stats){
            Ok(()) => {/* Nothing */},
            Err(e) => {
                use Errno::*;
                match e{
                    // SAFETY: We use a valid pointer and fd sense we create it ourselves
                    EFAULT|EBADF  => unsafe {core::hint::unreachable_unchecked()}
                    // SAFETY: we panic right after and the value is never refenced again
                    EOVERFLOW => {todo!()},
                    err => {
                        return Err(err)
                    }
                }

            }
        }
        // SAFETY: worst case we have a zeroed out stat struct
        let stats = unsafe {stats.assume_init()};

        if (stats.st_mode & 0o0040000 == 0o0040000){
            
            return Err(Errno::EISDIR)
        }
        use api::mman::*;
        // mmap!!
        // let _ = mmap(
            // None, size, proto, flags, fd, offset
        // )

        Ok(r)
    }

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
    pub const unsafe fn from_fd_unchecked(fd:RawFd)->Self{
        File {
            header: FILE_SIG,
            // SAFETY: caller guarantees this a valid file descriptor
            fd: ManuallyDrop::new(unsafe { OwnedFd::from_raw_fd(fd)}),
            lock:AtomicBool::new(false),
            owner_tid:Atomic::<pid_t>::new(0),
            // @Invariant: if locked and lock count is 0, this will underflow
            lock_count: UnsafeCell::new(0),
            buffer:None.into(),
            open: UnsafeCell::new(true),
            _marker:PhantomData
            
        }
    }

    /// Gives back the file descriptor
    /// 
    /// Use of the getter is due to preventing accidental modification
    /// of the file descriptor, as it should not be changed
    pub const fn fd(&self)->RawFd{
        (*self.fd).as_raw_fd()
    }

    /// Lock the file
    pub fn lock(&self)->FileLock<'_>{

        let tid = get_tid();
        match self.lock.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ) {
            Ok(false)=>{
                self.lock.store(true, Ordering::Release);
                self.owner_tid.store(tid, Ordering::Relaxed);
                
                // SAFETY: File is now locked
                unsafe {*self.lock_count.get() = 1};
                FileLock { inner: self }

            }
            Err(true) => { // Locked
                // See if we are apart of the same thread
                // We do relaxed as even if the file is outdated, we would write
                // to it to zero before changing, so if we are using the cpu cache, it would been cached
                if self.owner_tid.load(Ordering::Relaxed) == tid{
                    return FileLock { inner: self};
                }

                while self.lock.load(Ordering::Relaxed){
                    // @TODO: yield the thread
                    core::hint::spin_loop();
                }
                // SAFETY: We hold the lock on this thread
                unsafe {*self.lock_count.get() += 1};
                FileLock { inner: self }

            }
            // SAFETY: compare_exchange says that on success, `Ok()` would equal to current (false)
            Ok(true) => {unsafe {core::hint::unreachable_unchecked()}}
            _ => {unreachable!()}
        }
    }
    /// Tries to lock a file but fails if it is already locked by an another thread
    pub fn try_lock(&self)->Option<FileLock<'_>>{
        let tid = get_tid();
        match self.lock.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ){
            Ok(false) => {
                self.lock.store(true, Ordering::Release);
                self.owner_tid.store(tid, Ordering::Relaxed);
                
                // SAFETY: File is now locked
                unsafe {*self.lock_count.get() = 1};
                Some(FileLock { inner: self })
            },
            Err(true) => { // locked
                // we will simply see if this thread owns it at all
                if self.owner_tid.load(Ordering::Relaxed) == tid{
                    return Some(FileLock { inner: self});
                }
                None
            },
            // We may just return None, but we have a internal error so we should investigate
            _ => unreachable!()
        }
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
    pub fn try_clone(&self) ->Result<Self>{

        todo!("Get dup definition")
    }
    /// Does the same as dropping which closes the file. And returns any errors
    /// 
    /// # Errors
    /// Returns a tuple of `(Self,Errno)` so you can still operate on the File after a error as 
    /// 
    /// - EBADF  : fd is not a valid open file descriptor. This shouldn't occur in most cases
    /// - EINTR  : the close call was interrupted by a signal
    /// - EIO    : An I/O error occurred
    /// - ENOSPEC
    /// - EDQUOT : On NFS this will not occur until writes that occur after the quota exceeds
    /// 
    pub fn close_explicitly(self)->core::result::Result<(),(Self,Errno)>{
        // This makes sure we don't drop the value twice, which can cause undefined behaver
        let f = core::mem::ManuallyDrop::new(self);
        // SAFETY: we own the file descriptor
        match unsafe {api::io::stdio::close_fd(f.fd())}{
            Ok(_) => {
            
                Ok(())
            },
            Err(e) => Err((ManuallyDrop::into_inner(f), e))
        }
    }
    
}


impl api::io::Read for FileLock<'_>{
    fn read(&mut self, buf:&mut [u8])->api::Result<usize> {
        self.inner.fd.read(buf)
    }
}
impl api::io::Write for FileLock<'_>{
    fn write(&mut self,buf: &[u8]) -> api::Result<usize> {
        self.inner.fd.write(buf)
    }

    fn flush(&mut self) -> api::Result<()> {
        self.inner.fd.flush()
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
                    // We don't have the layout of the internal Kernel structures, but we want to handle this with the 
                    // alloc handler
                    ENOMEM => alloc::alloc::handle_alloc_error(core::alloc::Layout::new::<()>()),
                    
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
        // SAFETY: valid bit pattern
        // We do this so the header is now invalid if checked
        self._marker = unsafe {core::mem::zeroed()};
        if *self.open.get_mut(){
            // SAFETY: We are calling this in drop code, the
            // value should never be observed
            unsafe {ManuallyDrop::drop(&mut self.fd)}
        }
        
    }
}
/// A hold on a files lock
/// 
#[derive(Debug)]
pub struct FileLock<'fs>{
    inner:&'fs File
}
impl FileLock<'_>{
    pub fn write(&self,buf:&[u8])->Result<usize>{
        self.inner.fd.write(buf)
    }
    pub fn read(&self,buf:& mut[u8])->Result<usize>{
        self.inner.fd.read(buf)
    }
}
impl Drop for FileLock<'_>{
    #[expect(clippy::multiple_unsafe_ops_per_block, reason = "Same unsafety applies to both")]
    fn drop(&mut self) {
        // SAFETY: Valid pointer and we are the lock so we can access this lock
        unsafe {
            *self.inner.lock_count.get() -= 1;
            if *self.inner.lock_count.get() != 0{
                return; // Early return as we cannot unlock it yet
            }
        };
        // We set owner tid to 0 as a sentential value
        self.inner.owner_tid.store(0, Ordering::Relaxed);
        self.inner.lock.store(false, Ordering::Release);
    }
}
/// Follow posix standard that a lock cannot pass thread boundaries
impl !core::marker::Send for FileLock<'_> {}



macro_rules! define_outs {
    (
        $( $id:ident($fd:literal)),*
    
    ) => {
        $(
        
            // Safety: used for stdout, stderr,stdin, which aren't actual files
            pub static $id: File =  unsafe { File::from_fd_unchecked($fd)};

        )*
    };
}
define_outs!(
    STDIN(0),
    STDOUT(1),
    STDERR(2)
);