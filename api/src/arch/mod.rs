#[cfg(target_arch="x86_64")]
#[path ="./x86-64.rs"]
pub mod current;
// #[path ="./common.rs"]
pub mod common;
// Cannot use cause llvm bug with ebx reg
// #[cfg(any(target_arch = "x86",target_arch="x86_64"))]
// mod x86;

