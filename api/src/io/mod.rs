//! IO operations
//! 
mod stdio;
pub use stdio::{File,Mode};

pub trait Write {
    fn write(&mut self,buf: &[u8]) -> crate::Result<usize>;
    fn flush(&mut self) -> crate::Result<()>;
    fn write_all(&mut self, buf:&[u8]) -> crate::Result<()>;

    fn by_ref(&mut self) ->&mut Self{
        self
    }
}