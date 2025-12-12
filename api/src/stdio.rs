use bitflags::bitflags;

use crate::{prelude::*, syscall};




// This is passed out thru the api
// 
// This should be passed as a pointer outwards as `*mut File`
// But if some reason this was passed to a different version or different
// libc impl 
/// A Representation of a open file
/// 
/// This can be used with any file descriptor and not strictly a "file"
#[non_exhaustive]
#[repr(C)]
#[derive(Debug)]
pub struct File {
    /// Will be constant and should always be at the top of the struct
    magic:u32,
    ver:u32,
    fd: u64,
    mode:Mode
}
impl File{
    pub(crate) const MAGIC:u32 = 0xDEADBEEF;
    // incermint when something is added
    pub(crate) const CURRENT_VERSION:u32 = 0;
    /// Constructs a `FILE` from a file descriptor
    #[doc(alias = "fdopen")]
    pub const fn from_fd(fd:u64,mode:Mode)->Self{
        File {
            magic:File::MAGIC,
            ver: File::CURRENT_VERSION,

            fd,
            mode
        }
    }
    /// Gives back the file descriptor
    /// 
    /// Use of the getter is due to preventing accidental modification
    /// of the file descriptor, as it should not be changed
    pub const fn fd(&self)->u64{
        self.fd
    }
    /// Gives the magic variable
    /// 
    ///  # Const-ness
    /// This variable is guaranteed to be the value
    /// <code>0xDEADBEEF</code>, for rslibc FILE, but other
    /// libc implementations, or if rslibc's FILE becomes backwards incompatible
    /// (restructuring the fields where the version cannot be allowed)
    #[must_use]
    #[inline]
    pub fn get_magic(&self)->u32{
        self.magic
    }
    /// Get the version of the struct
    /// 
    /// # Const-ness
    /// While this variable should never change, we should not have
    /// the compiler assume it is always the case, as that would defeat the point of
    /// checking it 
    #[must_use]
    pub fn get_version(&self)->u32{
        self.ver
    }
    
}

fn write(
    fd:c_int,
    buffer:*const c_void,
    size: usize

)->crate::Result<isize>{ // ssize_t
    // SAFETY: Calls the syscall with the proper values
    let v = unsafe{
        syscall!(api_sys::SYS_write,
        fd,
        buffer,
         size)?
        };
        Ok(v as isize)
    }
// @TODO: use O_* consts
bitflags! {
    /// File mode options
    pub struct Mode:u8{
        const binary = 1;
        const read = 1<< 2; // sys::O_RDONLY;
        const write = 1 << 2;
        const append = 1 << 3;
        /// The equivalent to the `+`
        const update = 1 << 4;

    }
}
impl core::fmt::Debug for Mode{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        
        f.debug_struct("api::Mode")
            .field("binary", &self.contains(Mode::binary))
            .field("read", &self.contains(Mode::read))
            .field("write", &self.contains(Mode::write))
            .field("append", &self.contains(Mode::append))
            .field("update", &self.contains(Mode::update))
            .finish()
    }
}
// impl core::str::FromStr for Mode{
//     type Err;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }