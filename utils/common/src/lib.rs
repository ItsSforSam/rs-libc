//! Common (internal) functionality across rslibc crates
//! 
// This should NOT be touched by crt0 crate at all. As structures change or even simply
// a different
#![no_std]
#![feature(
    const_default,
    const_trait_impl,
)]
// #![cfg_attr(test, feature(freeze, negative_impls))] // Used for a compile test to ensure Freeze is used properly
use core::ffi::{c_int, c_long, c_ulong, c_void};

/// Shares the state of the program and functionalities of the program
#[derive(Debug)]
pub struct Libc{

    // This is only null if Libc is not yet initialized
    // Or if auxv is on a kernel which doesn't support it yet
    /// A pointer to the auxv structure. This is only null if 
    pub auxv: *const c_ulong,
    // This is so it gets marked !Freeze, and places it in writable memory
    // As we lazily initialize it once
    // #[doc(hidden)]
    /// PhantomData to give special neg impls like !Freeze
    // TODO: Why doesn't this work???
    secure:bool,
    /// Not used in threaded programs (see threading crate)
    /// But in single
    errno: core::cell::UnsafeCell<c_int>
}
#[repr(C)]
#[doc = "Elf_auxv_t"]
// #[derive(Debug)]
pub struct  ElfAuxv{
    /// The type for the Auxiliary Vector
    pub type_: c_long,
    /// The value of the vector. Depending on the type this is undefined (which is for the uninitialized form)
    /// # INVARIANT
    /// If 
    pub val: core::mem::MaybeUninit<a_un>
}
impl ElfAuxv{
    
    pub fn get_val(&self)->Option<AuxvVal>{
        match self.type_ {
            // LOTS OF MAGIC NUMBERS
            // they are taken from here: https://github.com/torvalds/linux/blob/1f318b96cc84d7c2ab792fcc0bfd42a7ca890681/include/uapi/linux/auxvec.h#L9-L42
            // @TODO: use proper constants instead of hardcoding the value
            0 | 1 => None,
            // SAFETY: Values of this type must have this type of value
            2|4|5|6|8|10|11|12|13|14|16|17|23|26|27|28|29|30|51 => Some(AuxvVal::Int(unsafe {self.val.assume_init().int})),
            7|15|25|31                      => Some(AuxvVal::Pointer(unsafe {self.val.assume_init().pointer})),
            9 => Some(AuxvVal::Function(unsafe {self.val.assume_init().function})),
            #[cfg(any(target_arch = "mips",target_arch = "powerpc"))]
            24 => Some(AuxvVal::Pointer(unsafe {self.val.assume_init().pointer})),
            // Reserved, we will silently say unknown due to not knowing the type unless debug is enabled (to prevent this from going un noticed for too long)
            18..=22 => {debug_assert!(false, "Reserved type of {}",self.type_); None}, 
            #[cfg(not(target_arch = "x86_64"))]
            rslibc_syscall::sys::AT_SYSINFO => Some(unsafe{self.val.assume_init().function}),
            // Debug assert to find out why value is invalid type; but we may be running in newer kernel so we cannot just always error
            _ => {debug_assert!(false, "Unknown type! got type of value {}",self.type_); None}
        }
    }
}
// impl Debug for ElfAuxv{
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         let type_name:&str = match self.type_ {
//             0 => "AT_NULL",
//             1 => "AT_IGNORE",
//             2 => "AT_EXECFD",
//             3 => "AT_PHDR",
//             4 => "AT_PHENT",
//             5 => "AT_PHNUM",
//             6 => "AT_PAGESZ",
//             7 => "AT_BASE",
//             8 => "AT_FLAGS",
//             9 => "AT_ENTRY",
//             // 10 =>
//         }
//     }
// }

/// The possible values for Auxiliary Vector
#[repr(C)]
#[expect(missing_debug_implementations, reason = "Unions cannot implement Debug without knowing the internal value")]
#[derive(Clone, Copy)] // Behave like C objects
pub union a_un{
    int: c_long,
    pointer: *mut c_void,
    /// The function pointer can be NULL
    function: Option<extern "C" fn()>,
}


#[derive(Debug)]
pub enum AuxvVal{
    Int(c_long),
    Pointer(*mut c_void),
    // NOTE: Can be null due to entrypoint being null
    /// A function pointer
    /// 
    /// Can be NULL due to 
    Function(Option<extern "C" fn()>) 
}
impl From<AuxvVal> for a_un{
    fn from(value: AuxvVal) -> Self {
        match value{
             AuxvVal::Int(v) => a_un {int:v},
             AuxvVal::Pointer(v) => a_un {pointer: v},
             AuxvVal::Function(v) => a_un {function:v}
        }
    }
}
impl ElfAuxv{

}
impl Libc{

    // pub fn new()->Self{

    // }
    
    pub fn get_errno(&self)->c_int{
        // SAFETY: gotten from valid pointer
        unsafe {*self.errno.get()}
    }
    pub fn get_errno_mut(&self)->c_int{
        self.errno.get_mut()
    }
    pub fn get_errno_raw(&self)->*mut c_int{
        self.errno.ge
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
        Libc { auxv: core::ptr::null(), _marker: core::marker::PhantomData}
    }
}
// #[cfg(test)]
// mod tests{
//     // use crate::Libc;

//     // trait Thawed {
    
//     // }
//     // impl<T: core::marker::Freeze> !Thawed for T  {}
//     // impl Thawed for Libc {} 
// }
