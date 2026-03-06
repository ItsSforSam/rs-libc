//! Provides all the types and constants
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_no_source)]


// macro_rules! def_mod{
//     ($($mod:ident)*) => {
//         $(
//             #[path = ::core::concat!(::core::env!("OUT_DIR"),"/",stringafiy!($mod))]
//             pub mod $mod;
//         )*

//     }
// }

include!(concat!(env!("OUT_DIR"),"/unistd.rs"));

// pub mod syscall{
//     include!(concat!(env!("OUT_DIR"),"/syscall.rs"));

    
// }
// // def_mod!(
//     types
// );