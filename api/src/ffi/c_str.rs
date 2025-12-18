use alloc::boxed::Box;


/// An ownable [`CStr`]
/// 
/// [`CStr`]:core::ffi::CStr
#[cfg_attr(not(doc), repr(transparent))]
pub struct CString{
    inner:Box<[u8]>
}

impl CString{
    pub fn new()->CString{
        todo!()
    }
}