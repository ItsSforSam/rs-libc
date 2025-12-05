use core::usize;

use crate::passing;


// This is passed out thru the api
// 
// This should be passed as a pointer outwards as `*mut File`
/// A Representation of a open file
#[non_exhaustive]
pub struct File{
    fd:usize
}
impl File{
    pub const fn new(fd:usize)->Self{
        File { fd }
    }
    
}
unsafe impl passing::PassOff for File{
     fn pass_off(self) -> *mut File{
        
    }
}