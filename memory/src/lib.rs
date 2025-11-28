//! Memory allocator for rslibc
//! 
//! This is one of the few crates that support 

#![no_std]

extern crate alloc;


use core::{ffi::c_void, num::NonZero, ptr::NonNull};


#[derive(Debug)]
pub struct Allocator{
    // This is a header
    heap:Option<NonNull<c_void>>

}



impl Allocator{
    pub const fn new() -> Allocator{

        Allocator {
            heap:None
        }


    }
    #[cfg_attr(any(test,miri), track_caller)]
    pub fn alloc(){

    }


    
}
#[derive(Debug)]
struct Node{
    
    isused:bool,
    size:NonZero<usize> // Zero not supported
}

// pub struct AllocLock<'a>{
    
// }







