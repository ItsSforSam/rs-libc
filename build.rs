#![allow(missing_docs,reason="build script")]
use std::path::PathBuf;
use std::env::var;
fn main(){
    println!("cargo::rustc-link-arg-cdylib=--entry=__libc_main");
}


fn get_arch_path()->PathBuf{
    let arch = var("CARGO_CFG_TARGET_ARCH").expect("TARGET_ARCH cfg not set, are you running this via build script?");
    let mut r = PathBuf::from("./arch");
    r.push(&arch);

    if !r.exists(){
        panic!("Not supported architecture {}", arch)
    }

    r
    

}