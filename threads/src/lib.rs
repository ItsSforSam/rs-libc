//! Handles the pthreads
#![no_std]
#![feature(
    unsafe_cell_access,
    negative_impls, // Used to guarantee
    push_mut
)]
use core::ffi::{c_int, c_uint};

use rslibc_syscall::{errno::Errno, syscall};
#[doc(inline)]
use rslibc_syscall::sys::pid_t;
use spin::RwLock;
// We need an allocator!
extern crate alloc;
use alloc::vec::Vec;
// static active pthreads
static ACTIVE_THREADS:RwLock<Vec<PThread>> = RwLock::new(Vec::new());
/// Posix Threads
#[derive(Debug)]
pub struct PThread{
    thread_id: pid_t,
    errno:core::cell::UnsafeCell<c_int>,

    // _marker:core::
}
// SAFETY: we provide locking the best we can do
// The only real data race is with errno which we try to keep it per thread
unsafe impl Sync for PThread {}
unsafe impl Send for PThread {}
impl PThread{
    /// Constructs a the pthread struct from a
    pub fn from_thread_id(thread_id:pid_t)->Self{
        PThread { thread_id, errno: core::cell::UnsafeCell::new(0) }
    }
    /// Get the errno value of the given pthread
    /// 
    /// # Result
    /// [`None`] - The value is an invalid [`Errno`] value, like 0
    /// [`Some`] - The current Errno of the given thread
    /// 
    /// If you want the value regardless of it's a valid [`Errno`], see [`get_errno_raw`]
    /// 
    /// [`get_errno_raw`]: PThread::get_errno_raw
    pub fn get_errno(&self)->Option<Errno>{
        // SAFETY: the value is received from a alive value
        let val = unsafe {*self.errno.get()} as c_uint;
        Errno::try_from(val).ok()
    }
    /// Get the internal Errno value in it's integer form
    /// 
    /// If you need a valid [`Errno`] value, see [`get_errno()`] method
    /// 
    /// This value may be an invalid Errno value due to the end developer setting the Errno to something that
    /// makes sense for their given program. As well if no errors have occurred yet, the value is by default 0
    /// 
    /// [`get_errno()`]: PThread::get_errno
    pub fn get_errno_raw(&self)->c_int{
        // SAFETY: the value is received from a alive value
        unsafe {*self.errno.get()}
    }

    pub fn swap_errno(&self,err:Errno)->c_int{
        // SAFETY: PThreads can only if RWlock is unlocked to writes
        unsafe {self.errno.replace(err.into())}
    }
    pub fn set_errno(&self){

    }
    pub fn tid(&self)->pid_t{
        self.thread_id
    }
}

impl core::cmp::PartialEq for PThread{
    #[doc(alias = "pthread_equal")]
    fn eq(&self, other: &Self) -> bool {
        return self.thread_id == other.thread_id;
    }
}
#[expect(clippy::multiple_unsafe_ops_per_block, reason="Included multiple unsafe comments")]
fn get_tid()->pid_t{
    // SAFETY: valid parameters
    unsafe {syscall!(SYS_gettid)
        // SAFETY: this syscall is always successful
        .unwrap_unchecked() as pid_t
    
    }
}

fn thread_self<'t>()->&'t PThread{
    let tid = get_tid();
    let threads = ACTIVE_THREADS.upgradeable_read();
    if threads.len() == 0{
        let mut up = threads.upgrade();
        return make_self(up);
    }
    for f in ACTIVE_THREADS.read().iter(){
        if f.tid() == tid{
            
        }
    }
    todo!();

    fn make_self(lock: spin::rwlock::RwLockUpgradableGuard<'_, alloc::vec::Vec<PThread>>)->&'static PThread{
        let ret = PThread::from_thread_id(get_tid());
        // lock.push(ret);
        let mut data = lock.upgrade();
        // let mut data = &*upg;
        // PANICS IF goes over ISIZE::MAX (How many threads does one person need)
        data.push(ret);
        // {
        // let mut i = lock.upgrade();
        // i.push(ret);

        // }
        
        if lock.len() == 1{
            // SAFETY: We checked
            return unsafe {lock.get(0).unwrap_unchecked()};
        }
        todo!()
        // return 
    }
}