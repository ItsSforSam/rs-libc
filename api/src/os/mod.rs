//! OS-specific functionality
//! 
//! This mirrors lot of the [`std::os`]
//! 
//! 
//! [`std::os`]: doc.rust-lang.org/std/os/index.html


#[cfg(unix)]
pub mod fd;
#[cfg(unix)]
pub mod unix;