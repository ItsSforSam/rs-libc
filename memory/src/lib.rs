//! Memory allocator for rslibc
//! 
//! This is one of the few crates that support 
// #![feature(allocator_api)]
#![no_std]
#![feature(ptr_alignment_type)]
#![feature(ptr_as_ref_unchecked)]
#![feature(ptr_as_uninit)]

extern crate alloc;


use core::alloc::Layout;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::{ffi::c_void, num::NonZero, ptr::{NonNull,Alignment},};

use alloc::alloc::{GlobalAlloc};
use alloc::ffi;
use api::mmap;
use spin::Mutex;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::ptr;
// use core::alloc::AllocError;
#[derive(Debug)]
pub struct Allocator{

    // This is a header
    node:spin::Mutex<Option<NonNull<Node>>> // Null should be None, like NonNull but could be mutated

}
#[derive(Debug)]
struct Node{

    free:bool,
    layout: core::alloc::Layout,
    pub(crate) next:Option<NonNull<Node>>, // Null should be None, like NonNull but that

}
// Does not need to drop as it simply is static only (currently)
// impl Drop for Node{
//     fn drop(&mut self) {
//         use core::mem::drop;
//         todo!()
//     }
// }
impl Node {
    const MIN_DATA_SIZE: usize = 1;

    fn get_data_offset(node:*const Node, alignment: usize) -> usize {
        // SAFETY: nodes always have at least MIN_DATA_SIZE with alignment 1
        let after_node_ptr:*const u8 =  unsafe{node.add(1)}.cast();
        after_node_ptr.align_offset(alignment)
    }
    fn get_data_ptr(node:*mut Node, alignment: usize) -> *mut u8 {
        let node_ptr:*mut u8 = node.cast();
        // SAFETY: nodes always have at least MIN_DATA_SIZE with alignment 1     
        unsafe{node_ptr.add(Node::get_data_offset(node, alignment))}
    }
    //None if it doesn't fit
    //Returned Some value will be >= layout.size()
    fn get_new_size_aligned(&self, layout:Layout) -> Option<usize> {
        let current_align = self.layout.align();
        let current_size = self.layout.size();

        let new_align = layout.align();
        let new_size = layout.size();

        let current_padding = Node::get_data_offset(self, current_align);
        let new_padding = Node::get_data_offset(self,new_align);
        let current_raw_size = current_padding+current_size;
        current_raw_size.checked_sub(new_padding).filter(|&capacity| capacity >= new_size)
    }

    //fails if self isn't free or if there isn't enough space
    fn split(&mut self,min_size:usize) -> bool {
        if !self.free {return false;}
        if let Some(n) = self.layout.size().checked_sub(size_of::<Node>()+Self::MIN_DATA_SIZE) && n>=min_size {
            //then split
            // SAFETY: checked if in bounds of allocation above
            let after = unsafe { Node::get_data_ptr(self, self.layout.align()).byte_add(min_size) };
            let padding = after.align_offset(align_of::<Node>());
            let new_node:&mut Node = unsafe { after.byte_add(padding).cast::<Node>().as_mut_unchecked() };
            //TODO: make sure that the new node fits into the data section even with the alignment
            if (unsafe { (new_node as *mut Node as *mut u8).add(size_of::<Node>()+Self::MIN_DATA_SIZE) } <after.add(self.layout.size()-min_size))

            *new_node = (Node { free: true, layout: unsafe { Layout::from_size_align_unchecked(1, 1) }, next: self.next });
            self.next = Some(unsafe { NonNull::new_unchecked(new_node) });

            return true;
        }

        false
    }
}



impl Allocator{

    pub const fn new() -> Allocator{
        
        Allocator {
            node:Mutex::new(None),
        }


    }
    #[cfg_attr(any(test,miri), track_caller)]
    pub fn alloc(&mut self,layout:Layout)-> Result<*mut c_void,AllocError> {
        let requested_size = layout.size();
        if requested_size == 0{
            return Err(AllocError::OutOfMemory);
        }
        let data = self.node.lock();
        match *data {
            Some(v) =>{
                //go thru every header and see which fits
                let mut current = Some(v);
                loop {
                    match current {
                        None => break,
                        Some(mut nonnull_node) => {
                            // SAFETY: iterating one by one so we only have one mut ref at a time
                            let node = unsafe{nonnull_node.as_mut()};
                            if node.free {

                                let new_size = node.get_new_size_aligned(layout);
                                if let Some(real_size) = new_size {
                                    // SAFETY: align follows rules. real_size might overflow isize but hopefully not
                                    node.layout = unsafe { Layout::from_size_align_unchecked(real_size, layout.align()) };
                                    node.split(real_size);
                                    node.free=false;
                                    return Ok(Node::get_data_ptr(node, layout.align()) as _);
                                }
                            }
                            current = node.next;
                        }
                    }
                }
                //couldn't find suitable Node
                let node = Self::alloc_node(layout, requested_size)?;
                return Ok(Node::get_data_ptr(node, layout.align()) as _);
            }
            //0x2
            //0 2 4 8 a c e 10
            //0x10
            //0 10 20 30
            None =>{
                let node = Self::alloc_node(layout, requested_size)?;
                return Ok(Node::get_data_ptr(node, layout.align()) as _);
            }
        }
        todo!()
    }
    
    fn alloc_node(layout:Layout, requested_size:usize) -> Result<*mut Node,AllocError> {
        //TODO test this alignment code
        let end = align_of::<Node>();
        let padding = if end<layout.align() {
            layout.align()-end
        } else {
            0
        };
        let real_size = requested_size+padding+size_of::<Node>();
        let v = Allocator::alloc_inner(real_size);
        if v.is_null(){
            return Err(AllocError::OutOfMemory);
        }

        // Node is a heder and exists before every allocation in memory
        // SAFETY: checked if null above
        let node_ptr:NonNull<Node> = unsafe {NonNull::new_unchecked(v as _)};
        // SAFETY: pointer is convertible to reference
        let uninit_node = unsafe { node_ptr.as_uninit_mut() };
        let node = uninit_node.write(
        Node{
                free:false,
                // SAFETY: align follows rules. real_size might overflow isize but hopefully not
                layout:unsafe { Layout::from_size_align_unchecked(real_size, layout.align()) },
                next:None,
            }
        );
        Ok(node)
    }
    
    /// Force an allocation
    /// 
    /// Does not pass the allocator, just do the raw allocation
    fn alloc_inner(size:usize) -> *mut c_void{
        api::mmap(
            ptr::null_mut(), // Null basically tell the Kernel "idc you chose!"
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
        f.write_str("An Allocation error occurred")
    }
}

impl core::error::Error for AllocError{}

// Safety: Supposed to be thread safe all the time anytime
unsafe impl Sync for Allocator{}
// Safety: Same as above
unsafe impl Send for Allocator{}