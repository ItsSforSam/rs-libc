//! Contains architecture-specific code
#![expect(clippy::allow_attributes_without_reason, reason = "Reduce redundancy")]
// #[path ="./common.rs"]
pub mod common;
// Cannot use cause llvm bug with ebx reg
// #[cfg(any(target_arch = "x86",target_arch="x86_64"))]
// mod x86;
#[cfg(target_arch="x86_64")]
#[expect(missing_docs)]
pub mod x86_64;

/// Re-exports the current architecture
///
#[cfg_attr(target_arch = "x86_64", doc = "In this case it's x86-64")]
pub mod current{
    cfg_if::cfg_if!{
        if #[cfg(target_arch="x86_64")]{
            #[doc(inline)]
            pub use super::x86_64::*;
        } else{
            // Fail as we need the modules regardless
            compile_error!("No architecture module supplied")
        }
    }
}
