use std::ffi::c_void;
use std::path::PathBuf;
use std::env::var;
fn main(){
    cc::Build::new()
        .cpp(false)
        .flag("-Wa")
        .file("");
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