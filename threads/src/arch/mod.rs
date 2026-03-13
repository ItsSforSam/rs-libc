//! Architecture specific functionality
// This is the resource for this
#[cfg(target_arch = "x86_64")]
pub mod x86_64;

/// The current architecture
/// 
/// This just reexports the represented module
pub mod current{
   #[cfg(target_arch = "x86_64")]
   #[doc(inline)]
   pub use super::x86_64::*;
}