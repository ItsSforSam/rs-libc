//! IO operations
//! 
pub mod stdio;
pub use stdio::{File,Mode};

use crate::errno::Errno;

/// Trait for objects which are byte-oriented sincks
/// 
/// This follows closely to the schematics as [`std::io::Write`]
/// 
/// [`std::io::Write`]:https://doc.rust-lang.org/std/io/trait.Write.html
pub trait Write {
    /// Writes a buffer into the writer
    /// 
    /// Returns how many bytes left as [`Ok(n)`]
    /// 
    /// [`Ok(n)`]: Ok
    fn write(&mut self,buf: &[u8]) -> crate::Result<usize>;
    // Flushes the output stream, ensuring all internally buffered contents make their destination
    fn flush(&mut self) -> crate::Result<()>;

    /// Writes an entire buffer to the writer until an error occurs or a non-[`EINTR`] error occurs
    /// 
    /// # Additional Error
    /// The default implementation will return [`EFBIG`] error if a file is "full".
    /// This will occur if Ok(0) is returned with more in the buffer.
    /// 
    /// [`EINTR`]: Errno::EINTR
    /// [`EFBIG`]: Errno::EFBIG
    fn write_all(&mut self, mut buf: &[u8])->crate::Result<()>{
        // ports this: https://doc.rust-lang.org/src/std/io/mod.rs.html#1875-1887
        while !buf.is_empty(){
            match self.write(buf){
                Ok(0) => {
                    // If 0 and there is still content left it means the file is fill
                    // In std they return a unexpected EOF error (or more accurately a WRITE_ALL_EOF error)
                    // We don't have this, so an error of "this file is too large" will work currently
                    return Err(Errno::EFBIG);
                }
                Ok(n) => buf = &buf[n..],
                Err(ref e) if *e == Errno::EINTR => {/* Nothing! Just continue */},
                Err(e) => return Err(e)
            }
        }
        Ok(())
    }
}
/// Allows reading bytes from a source
/// 
/// This follows closely to the schematics as [`std::io::Read`]
/// 
/// [`std::io::Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
pub trait Read {
    /// Pull data from source and 
    fn read(&mut self, buf:&mut [u8])->crate::Result<usize>;
}