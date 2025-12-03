//! Memory allocator for rslibc
//! 
//! This is one of the few crates that support 
// #![feature(allocator_api)]
#![no_std]
#![feature(ptr_alignment_type,ptr_as_ref_unchecked,ptr_as_uninit)]

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
    free: bool,
    layout: core::alloc::Layout,
    pub(crate) next:Option<NonNull<Node>>, // None if there is no next
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
    /// None if it doesn't fit
    /// Returned `Some` value will be `>= layout.size()`
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

    /// Fails if self isn't free or if there isn't enough space
    fn split(&mut self,min_size:usize) -> bool {
        if !self.free {return false;}
        if let Some(n) = self.layout.size().checked_sub(size_of::<Node>()+Self::MIN_DATA_SIZE) && n>=min_size {
            //then split
            // SAFETY: checked if in bounds of allocation above
            let after = unsafe { Node::get_data_ptr(self, self.layout.align()).byte_add(min_size) };
            let padding = after.align_offset(align_of::<Node>());
            let new_node = {
                // SAFETY: aligned
                let ptr = unsafe { after.byte_add(padding)}.cast::<Node>();
                // SAFETY: convertible to reference
                let uninit = unsafe{ptr.as_uninit_mut()};
                // SAFETY: known not to be null
                unsafe { uninit.unwrap_unchecked() }
            };
            let new_node_ptr = new_node.as_ptr();
            let new_node_end = new_node_ptr as usize + (size_of::<Node>()+Self::MIN_DATA_SIZE) ;
            let real_after = after as usize + (self.layout.size()-min_size);
            //TODO: make sure that the new node fits into the data section even with the alignment
            if new_node_end != real_after {return false;}

            let next_node = new_node.write(
                // SAFETY: constants are valid align and don't overflow
                Node { free: true, layout: unsafe { Layout::from_size_align_unchecked(1, 1) }, next: self.next }
            );
            // SAFETY: next_node/new_node is not null
            self.next = Some(unsafe { NonNull::new_unchecked(next_node) });

            return true;
        }

        false
    }
    /// Fails if it isn't free or next isn't free
    fn join(&mut self) -> bool {
        if !self.free {return false;}
        if let Some(mut next_ptr) = self.next {
            // SAFETY: Node.next always assumed to be valid
            let next = unsafe { next_ptr.as_mut() };
            if !next.free {return false;}
            // SAFETY: it has a block after so all the data in between is valid
            let end = unsafe { Node::get_data_ptr(self, self.layout.align()).add(self.layout.size()) };
            let end_next_padding = end.align_offset(align_of::<Node>());

            self.next = next.next;
            //this makes the next block no longer valid
            let total_size = end_next_padding+size_of::<Node>()+Node::get_data_offset(next, next.layout.align())+next.layout.size();

            // SAFETY: align follows rules. size might overflow isize but hopefully not
            self.layout = unsafe { Layout::from_size_align_unchecked(self.layout.size()+total_size, self.layout.align()) };
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
    //use *mut u8 to match GlobalAlloc
    
    pub fn inner_alloc(&self,layout:Layout)-> Result<*mut u8,AllocError> {
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
                                    return Ok(Node::get_data_ptr(node, layout.align()));
                                }
                            }
                            current = node.next;
                        }
                    }
                }
                // Can't find a free Node with an appropriate size. Allococate!
                let node = Self::alloc_node(layout, requested_size)?;
                Ok(Node::get_data_ptr(node, layout.align()))
            }

            None =>{
                let node = Self::alloc_node(layout, requested_size)?;
                Ok(Node::get_data_ptr(node, layout.align()))
            }
        }
    }
    
    fn alloc_node(layout:Layout, requested_size:usize) -> Result<*mut Node,AllocError> {
        // TODO test this alignment code
        // TODO allocate a page(get pagesize at runtime) if the allocation fits (so when freed, other smaller allocations can use), if not allocate full size
        let end = align_of::<Node>();
        let padding = if end<layout.align() {
            layout.align()-end
        } else {
            0
        };
        let real_size = requested_size+padding+size_of::<Node>();
        let v = Allocator::raw_alloc(real_size);
        if v.is_null(){
            return Err(AllocError::OutOfMemory);
        }

        // Node is a header and exists before every allocation in memory
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
    fn raw_alloc(size:usize) -> *mut c_void{
        api::mmap(
            ptr::null_mut(), // Null basically tell the Kernel "idc you chose!"
             size,
             api::MMapProt::Read | api::MMapProt::Write, 
             api::MMapFlags::Anonymous | api::MMapFlags::Private, 
             -1,
            0
        )
    }
    pub fn inner_dealloc(&self, ptr: *mut u8) {
        let lock = self.node.lock();
        if let Some(first) = *lock {
            let mut current = Some(first);
            loop {
                match current {
                    None => break,
                    Some(mut nonnull_node) => {
                        // SAFETY: convertible to ref
                        let node = unsafe{nonnull_node.as_mut()};
                        if Node::get_data_ptr(node, node.layout.align())==ptr {
                            node.free=true;
                            //join all the free nodes next to each other
                            //fixes fragmentation
                            loop {if !node.join() {break;}}
                        }
                        current=node.next;
                    }
                }
            }
        }
    }
}


// pub struct AllocLock<'a>{
    
// }
// Safety: Read GlobalAlloc's safety doc
unsafe impl GlobalAlloc for Allocator{
    #[cfg_attr(any(test,miri), track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        match self.inner_alloc(layout) {
            Ok(v) => v as _,
            Err(_) => ptr::null_mut(),
        }
    }
    #[cfg_attr(any(test,miri), track_caller)] // even without panics, this helps for Miri backtraces
    unsafe fn dealloc(&self, pointer: *mut u8, _: core::alloc::Layout) {
        self.inner_dealloc(pointer as _)
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
