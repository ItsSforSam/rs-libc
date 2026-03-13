//! Common (internal) functionality across rslibc crates
//! 
// This should NOT be touched by crt0 crate at all. As structures change or even simply
// a different
#![no_std]
#![feature(
    const_default,
    const_trait_impl,
    linkage
)]
// #![cfg_attr(test, feature(freeze, negative_impls))] // Used for a compile test to ensure Freeze is used properly
use core::{ffi::{c_int, c_ulong}, ptr::NonNull};
pub mod auxv;
/// Shares the state of the program and functionalities of the program
/// 
/// This is a semi-singleton
#[derive(Debug)]
pub struct Libc{

    // This is only null if Libc is not yet initialized
    // Or if auxv is on a kernel which doesn't support it yet
    /// A pointer to the auxv structure. This is only null if 
    auxv: *const c_ulong,
    // This is so it gets marked !Freeze, and places it in writable memory
    // As we lazily initialize it once
    // #[doc(hidden)]
    /// Determines is secure mode
    secure:bool,
    /// Not used in threaded programs (see threading crate)
    /// But in single threaded contexts it is used for errno
    // Due to how we initialize it properly
    // we need !Freeze to put Libc in writable memory
    errno: core::cell::UnsafeCell<c_int>
}
// Only time it is not is in multithreaded, but should use an alternative if that's the case
unsafe impl core::marker::Send  for Libc {}
unsafe impl core::marker::Sync  for Libc {}
impl Libc{
    
    pub fn new()->Self{

    }
    /// # SAFETY
    /// This pointer cannot be written to under any reason
    #[must_use = "This function has no side effects"]
    pub fn get_auxv_ptr(&self)->Option<NonNull<c_ulong>>{
        NonNull::new(self.auxv as *mut c_ulong)
    } 
    #[doc(hidden)]
    // Used so main package can call default
    pub const fn default()->Self{
        Default::default()
    }
    #[must_use = "This function has no side effects"]
    pub fn is_secure(&self)->bool{
        self.secure
    }
    #[must_use = "This function has no side effects"]
    pub fn get_errno(&self)->c_int{
        // SAFETY: gotten from valid pointer
        unsafe {*self.errno.get()}
    }
    /// # SAFETY
    /// Guarantee that there are no other references to errno object
    #[must_use = "This function has no side effects"]
    #[expect(clippy::mut_from_ref, reason = "There is no way to get the global Libc in mut and this value should only be available in single threaded contexts")]
    pub unsafe fn get_errno_mut(&self)->&mut c_int{
        // SAFETY: gotten from valid reference
        unsafe {&mut *self.errno.get()}
    }
    #[must_use = "This function has no side effects"]
    pub fn get_errno_raw(&self)->*mut c_int{
        self.errno.get()
    }
    /// Get a set value from the Auxiliary Vector
    /// 
    /// # Returns
    /// None - If entry is not found or no aux vector available 
    #[doc = "getauxval"]
    #[cfg(unix)]
    pub fn get_aux_val(&self,type_: c_ulong)->Option<c_ulong>{
        // This is based off of this which is licensed on MIT (which is compatible with Apache to my knowledge (not legal advice))
        // https://github.com/torvalds/linux/blob/651690480a965ca196ce42d4562543f3e61cb226/tools/include/nolibc/sys/auxv.h
        // 
        // https://refspecs.linuxfoundation.org/LSB_1.3.0/IA64/spec/auxiliaryvector.html

        if self.auxv.is_null(){
            log::debug!("Auxiliary Vector pointer null");
            return None
        }
        let mut auxv:*const c_ulong = self.auxv;
        loop{
            // SAFETY: we got the pointer from the Kernel, which should be valid memory. Up to two zero longs next to each other
            let a = unsafe{*auxv};
            
            
            if a == 0{ // AT_NULL
                // value is undefined so it's not dereferenced until after
                return None
            }
            if a==1{ // AT_IGNORE
                // "The value in a_un is undefined and should be ignored"
                // This may not matter as what I look
                continue;
            }
            // Do we need to put a compiler fence for this?
            // SAFETY: Same as deref-ing above
            let v = unsafe { *(auxv.wrapping_add(1)) };
            if a == type_{
                // ret = v;
                return Some(v);
            }

            auxv = auxv.wrapping_add(2);

        }
        
    }
}
impl const Default for Libc{
    fn default() -> Self {
        Libc { auxv: core::ptr::null(), secure:false, errno:core::cell::UnsafeCell::new(0) }
    }
}
