//! Memory allocator for rslibc
//! 
//! This is one of the few crates that support 
// #![feature(allocator_api)]
#![no_std]

extern crate alloc;



use core::{ffi::c_void, num::NonZero, ptr::NonNull};

use alloc::alloc::{GlobalAlloc};
use alloc::ffi;
use api::mmap;
use core::sync::atomic::{AtomicBool,Ordering};
use core::ptr;
// use core::alloc::AllocError;
#[derive(Debug)]
pub struct Allocator{

    // This is a header
    node:Option<*mut Node>, // Null should be None, like NonNull but could be mutated
    // phantom:PhantomData<&mut Node>
}
#[derive(Debug)]
struct Node{
    
    isused:AtomicBool,
    pub size:NonZero<usize>, // Zero not supported for Global Allocator, but if we
    pub(crate) next:Option<*mut Node>, // Null should be None, like NonNull but that

    data:*mut c_void
}
// Does not need to drop as it simply is static only (currently)
// impl Drop for Node{
//     fn drop(&mut self) {
//         use core::mem::drop;
//         todo!()
//     }
// }



impl Allocator{

    pub const fn new() -> Allocator{
        
        Allocator {
            node:None
        }


    }
    #[cfg_attr(any(test,miri), track_caller)]
    pub fn alloc(&mut self,size:usize)-> *mut c_void {
        if size == 0{
            return core::ptr::null_mut();
        }
        match self.node{
            Some(v) =>{todo!()}

            None =>{
                let v = Allocator::alloc_inner(size);
                if v.is_null(){
                    return v;
                }
                let d = Node{
                    isused:AtomicBool::new(true),
                    // Safety: checked above
                    size:unsafe{NonZero::new(size).unwrap_unchecked()},
                    next:None,
                    data:v
                };
            }
        }
        todo!()
    }
    /// Force an allocation
    /// 
    /// Does not pass the allocator, just do the raw allocation
    fn alloc_inner(size:usize) -> *mut c_void{
        api::mmap(
            0 as _, // Null basically tell the Kernel "idc you chose!"
             size,
             api::MMapProt::Read | api::MMapProt::Write, 
             api::MMapFlags::Anonymous | api::MMapFlags::Private, 
             -1,
            0
        )
    }
}


// pub struct AllocLock<'a>{
    
// }
// Safety: Read GlobalAlloc's safety doc
unsafe impl GlobalAlloc for Allocator{
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
        
    }
    
    unsafe fn dealloc(&self, pointer: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }

}
/// An Allocation error has occurred
/// 
/// Usually used for reason of error.
#[derive(Debug)]
#[non_exhaustive]
pub enum AllocError{
    /// System is out of memory.
    /// 
    /// This when not handled will be passed to the the oom handler which will panic
    OutOfMemory,
    /// Returned when size of 0 was passed to
    /// 
    /// This is effectively a no-op
    TooSmall
}
impl core::fmt::Display for AllocError{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use AllocError::*;
        match self{
            // These should not be relied upon for grep-ing
            OutOfMemory => f.write_str("An allocation could not succeed. Out of memory"),
            
            TooSmall    => f.write_str("Size of zero was specified. No-allocation occurred"),
        }
        
    }
}

impl core::error::Error for AllocError{}

// Safety: Supposed to be thread safe all the time anytime
unsafe impl Sync for Allocator{}
// Safety: Same as above
unsafe impl Send for Allocator{}