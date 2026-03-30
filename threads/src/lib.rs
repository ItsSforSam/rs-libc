//! Handles the pthreads
#![no_std]
#![no_main]
#![feature(
    unsafe_cell_access,
    negative_impls, // Used to guarantee
)]
pub mod arch;

use core::{ffi::{c_int, c_uint}, mem::offset_of, ptr::NonNull};

use rslibc_syscall::{errno::Errno, syscall};
#[doc(inline)]
use rslibc_syscall::sys::pid_t;
/// Posix Threads
#[derive(Debug)]
#[repr(C)] // The ABI is not stable, but we need stuff to be laid out properly
pub struct PThread{
    /// A self pointer referring to itself
    pself: *mut Self,
    /// The Thread ID (tid) 
    /// 
    /// This is refers to a specific thread 
    thread_id: pid_t,
    errno:core::cell::UnsafeCell<c_int>,

    /// Used to mark special data
    /// 
    /// This needs to be pinned due to passing this pointer to C land
    _marker: core::marker::PhantomData<core::marker::PhantomPinned>
    // _marker:core::
}
/// # SAFETY
/// This will only be changed by [`__rsinit_thread_lib`] function
static mut __PANIC_IMPL:Option<extern "Rust" fn(&core::panic::PanicInfo) ->!> = None;

/// Initializes threadlibs
/// 
#[doc = include_str!("../../meta/docs/abi-breakage.md")]
/// 
/// # SAFETY
/// 
/// Shouldn't be called directly
/// ABI can break as an internal function
#[unsafe(no_mangle)]
#[expect(improper_ctypes_definitions, reason ="It's fine dw about it")]
pub unsafe extern "C" fn __rsinit_thread_lib(panic_handler:extern "Rust" fn(&core::panic::PanicInfo)->!)->u8{
    // Safety: Caller guarantees safety guarantees are upheld
    unsafe {__PANIC_IMPL = Some(panic_handler)}
    todo!();
}

// // SAFETY: we provide locking the best we can do
// // The only real data race is with errno which we try to keep it per thread
// unsafe impl Sync for PThread {}
// // SAFETY: Same as above
// unsafe impl Send for PThread {}
impl PThread {
    /// Returns the current PThread struct
    pub fn current()->Option<&'static mut Self>{
        // SAFETY: correct offset
        let p = unsafe { crate::arch::current::get_tp(offset_of!(Self,pself))} as *mut Self;
        if p.is_null(){
            return None;
        }
        todo!()
    }
    /// Constructs a the pthread struct from a
    pub fn from_thread_id(thread_id:pid_t)->Self{
        PThread { thread_id, errno: core::cell::UnsafeCell::new(0),_marker:core::marker::PhantomData, pself:core::ptr::null_mut()}
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
    /// Swap the errno value
    /// 
    /// Returns the old errno
    pub fn swap_errno(&self,err:Errno)->c_int{
        // SAFETY: PThreads can only if RWlock is unlocked to writes
        unsafe {self.errno.replace(err.into())}
    }
    /// Sets the errno 
    pub fn set_errno(&self, err: Errno){
        // SAFETY: 
        unsafe {*self.errno.get() = err.into()}
    }
    /// Returns the thread id for the specific PThread struct
    #[must_use]
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



#[panic_handler]
fn panic_handler(i:&core::panic::PanicInfo)->!{
    // SAFETY: No race condition can occur as this function is changed
    // before threads are started
    match unsafe {__PANIC_IMPL}{
        Some(f)=>f(i),
        None => loop{}
    }
}

// fn thread_self<'t>()->&'t PThread{
//     let tid = get_tid();
//     let threads = ACTIVE_THREADS.upgradeable_read();
//     if threads.len() == 0{
//         let mut up = threads.upgrade();
//         return make_self(up);
//     }
//     for f in ACTIVE_THREADS.read().iter(){
//         if f.tid() == tid{
            
//         }
//     }
//     todo!();

//     fn make_self(lock: spin::rwlock::RwLockUpgradableGuard<'_, alloc::vec::Vec<PThread>>)->&'static PThread{
//         let ret = PThread::from_thread_id(get_tid());
//         // lock.push(ret);
//         let mut data = lock.upgrade();
//         // let mut data = &*upg;
//         // PANICS IF goes over ISIZE::MAX (How many threads does one person need)
//         data.push(ret);
//         // {
//         // let mut i = lock.upgrade();
//         // i.push(ret);

//         // }
        
//         if lock.len() == 1{
//             // SAFETY: We checked
//             return unsafe {lock.get(0).unwrap_unchecked()};
//         }
//         todo!()
//         // return 
//     }
// }
