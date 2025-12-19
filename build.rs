#![allow(missing_docs,reason="build script")]
use std::path::PathBuf;
use std::env::var;
use std::fs;

fn main(){
    let out = PathBuf::from(var("OUT_DIR").unwrap());
    println!("cargo::rustc-link-arg-cdylib=--entry=__libc_main");
    fs::write(out.join("meta.rs"), gen_meta()).unwrap();

}
// fn get_arch_path()->PathBuf{
//     let arch = var("CARGO_CFG_TARGET_ARCH").expect("TARGET_ARCH cfg not set, are you running this via build script?");
//     let mut r = PathBuf::from("./arch");
//     r.push(&arch);

//     if !r.exists(){
//         panic!("Not supported architecture {}", arch)
//     }

//     r
    

// }
fn gen_meta()->String{
    let version = std::env::var("CARGO_PKG_VERSION").unwrap();
    let r = format!(
        r#"
// Meta information about rslibc
// This is for info to be included in the 

use core::ffi::c_char;

#[unsafe(link_section = ".comment.rslib_meta")]
#[unsafe(export_name = "rs_version")]
#[used]
static NOTE_VERSION:ConstVer = ConstVer(c"{version}".as_ptr());
#[repr(transparent)]
struct ConstVer(*const c_char);

unsafe impl core::marker::Send for ConstVer {{}}
unsafe impl core::marker::Sync for ConstVer {{}}
        "#
    );
    r
}

