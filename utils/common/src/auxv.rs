use rslibc_syscall::sys;
use core::{ffi::{c_long, c_void}, fmt::Debug};
#[repr(C)]
#[doc = "Elf_auxv_t"]
// #[derive(Debug)]
pub struct  ElfAuxv{
    /// The type for the Auxiliary Vector
    ///
    /// 
    pub type_: c_long,
    /// The value of the vector. Depending on the type this is undefined (which is for the uninitialized form)
    /// # INVARIANT
    /// If not initialized and is not AT_NULL
    pub val: core::mem::MaybeUninit<a_un>
}

impl core::fmt::Debug for ElfAuxv{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let atype = match TryInto::<AuxType>::try_into(self.type_){
            Ok(v) => {v},
            Err(_) => {unreachable!("Unknown AuxType; contact the rslibc maintainers")}
        };
        
        f.debug_struct("ElfAuxv")
        .field("type", &atype)
        // We get the value due to the internal value being maybe uninitialized
        .field("value", &self.get_val())
        .finish()
    }
}
impl ElfAuxv{
    /// Returns the Auxiliary Vector member's type
    /// 
    /// # Returns
    /// * `None` if the internal value is not a valid member of the [`AuxType`] enum
    pub fn get_type(&self)->Option<AuxType>{
        AuxType::try_from(self.type_)
    }
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
        
//     }
// }

macro_rules! make_auxtype {
    (
        impl AuxType {
            $(
                $(#[$attr:meta])*
                // #[doc(alias = stringify!($actual_name))]
                $name:ident = $actual_name:ident;
            )*
        }
    ) => {
    
        
    #[derive(Debug,Clone)]
    #[repr(i64)]
    // #[expect]
    pub enum AuxType {
        $(
            $name = sys::$actual_name as ::core::ffi::c_long, // if c_long is not i64 this will error
        )*
    }
     impl AuxType{
        /// This is the equivalent to [`TryFrom`]
        /// 
        /// [`TryFrom`]: core::convert::TryFrom
        #[doc(hidden)]
        pub fn try_from(val:c_long)->Option<AuxType>{
            match val{
                $(
                    v if v == sys::$actual_name as c_long => Some(AuxType::$name),
                
                )*
                _ => None
            }
        }
    }
    impl core::cmp::PartialEq for AuxType{
        fn eq(&self,other:&Self)-> bool{
            self.clone() as c_long == other.clone() as c_long
        }
    }

    impl TryFrom<c_long> for AuxType{
        /// No data can be shared, this error may change into a more concrete error
        type Error = ();
        fn try_from(value:c_long)->Result<Self,Self::Error>{
            AuxType::try_from(value).ok_or(())
        }
    }
    };
}
make_auxtype!{
    impl AuxType{
        // Man it was fun to just regex stuff
        NULL               = AT_NULL;
        IGNORE             = AT_IGNORE;
        EXECFD             = AT_EXECFD;
        PHDR               = AT_PHDR;
        PHENT  		       = AT_PHENT;
        PHNUM  		       = AT_PHNUM;
        PAGESZ  	       = AT_PAGESZ;
        BASE  		       = AT_BASE;
        FLAGS  		       = AT_FLAGS;
        ENTRY  		       = AT_ENTRY;
        NOTELF  	       = AT_NOTELF;
        UID  		       = AT_UID;
        EUID  		       = AT_EUID;
        GID  		       = AT_GID;
        EGID  		       = AT_EGID;
        PLATFORM  	       = AT_PLATFORM;
        HWCAP  		       = AT_HWCAP;
        CLKTCK  	       = AT_CLKTCK;
        Secure             = AT_SECURE;
        BasePlatform      = AT_BASE_PLATFORM;
        RANDOM             = AT_RANDOM;
        HWCAP2             = AT_HWCAP2;
        RSEQ_FEATURE_SIZE  = AT_RSEQ_FEATURE_SIZE;
        RSEQ_ALIGN         = AT_RSEQ_ALIGN;
        HWCAP3             = AT_HWCAP3;
        HWCAP4             = AT_HWCAP4;
        EXECFN             = AT_EXECFN;
        MINSIGSTKSZ  	   = AT_MINSIGSTKSZ;

    }

    
}


#[derive(Debug)]
pub struct AuxiliaryVector{
    inner:[ElfAuxv]
}



impl AuxiliaryVector{
    /// Returns a unsized type to represent a Auxiliary Vector
    /// # SAFETY
    /// * `pointer` is non-null
    /// * The value is null terminated with [`AuxType::NULL`]
    /// * The data which `pointer` points to, up to [`AuxType::NULL`] is within the same allocation
    /// * `pointer` is properly aliened
    pub unsafe fn from_ptr<'a>(pointer:*const ElfAuxv)->&'a AuxiliaryVector{
        use core::ptr::read;
        let mut len: usize = 0;
        let mut p =pointer;
        loop{
            // We gotta check for the null terminater
            // SAFETY: user safely guarantees this is valid
            let data = unsafe {read(p)};
            match data.get_type(){
                
                Some(v) if v == AuxType::NULL => {
                    // SAFETY: caller guarantees that pointer up to NULL is valid and we correctly count up to null
                    let ret:&AuxiliaryVector = unsafe {core::mem::transmute(core::slice::from_raw_parts(pointer, len))};
                    return ret;
                },
                None | Some(_) => {
                    len += 1;
                    p= p.wrapping_add(1);
                    continue;
                }
            }
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