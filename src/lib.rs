#![no_std]
#![no_main]
#![allow(internal_features, reason ="Uses lang item of eh_personality")]
#![feature(linkage,lang_items,alloc_error_handler)]
mod panicking;
mod rt;
pub mod env;
extern crate alloc;

#[global_allocator]
static ALLOCATOR: memory::Allocator = memory::Allocator::new();
