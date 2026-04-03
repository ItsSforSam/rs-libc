#![allow(missing_docs,reason="build script")]
use std::path::PathBuf;
use std::env::{var, var_os};
use std::fs;

fn main(){
    let out = PathBuf::from(var("OUT_DIR").unwrap());
    // for (key, value) in std::env::vars_os(){
    //      println!("{:?} = {:?}",key,value);
    // }
    if setup_dyn(){
        println!("cargo::rustc-link-arg-cdylib=--entry=__libc_main");
        println!("cargo::rustc-cfg=linkage=\"dynamic\"")
    }else{
        println!("cargo::rustc-cfg=linkage=\"static\"")
    }
    
    
    fs::write(out.join("meta.rs"), gen_meta()).unwrap();
}

/// Get if this is dynamically linked
fn setup_dyn()->bool{
    // https://doc.rust-lang.org/rustc/codegen-options/index.html#relocation-model

    
    if var_os("CARGO_PRIMARY_PACKAGE").is_none(){
        return false;
    }
    let is_dyn:bool = match var("CARGO_CFG_RELOCATION_MODEL").expect("cfg(relocation_model) missing").as_str() {
        "static" => false,
        "dynamic-no-pic" => false, // ??? only used on Darwin/Mac rarely
        "pic" => true,
        "pie" => false,
        other => unimplemented!("Unresolved relocation model of `{}`",other)
    };

    // if var("CARGO_CFG_TARGET_FEATURE").unwrap().contains("crt-static"){

    // }

    return is_dyn;
    
    
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
    let rust_version:String;
    if let Some(ver) =  version_check::Version::read(){
        rust_version = format!("{ver}")
    } else{
        rust_version = "???".to_string(); // shouldn't be considered stable
    }

    let r = format!(
        r#"
// Meta information about rslibc
// This is for info to be included in the 

use core::ffi::c_char;

#[unsafe(link_section = ".comment.rslib_meta")]
#[unsafe(export_name = "rs_version")]
#[used]
static NOTE_VERSION:ConstVer = ConstVer(c"{version}".as_ptr());
#[unsafe(link_section = ".comment.rslib_meta")]
#[unsafe(export_name = "rustc_version")]
#[used]
static RUSTC_VERSION:ConstVer = ConstVer(c"{rust_version}".as_ptr());

#[repr(transparent)]
struct ConstVer(*const c_char);

unsafe impl core::marker::Send for ConstVer {{}}
unsafe impl core::marker::Sync for ConstVer {{}}
        "#
    );
    r
}

