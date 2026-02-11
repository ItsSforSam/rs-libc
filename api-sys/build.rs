#![feature(io_error_more)]
use std::path::{Path, PathBuf};
use std::env;

// extern crate bindgen;
// fn is_linux(){

// }

fn main(){
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=wrapper.h");
    println!("cargo::rerun-if-changed=../include");
    let out_path:PathBuf = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR env not set, are you using cargo?"));
    gen_bindings("wrapper.h", "unistd.rs", &out_path);
}
// use bindgen::BindgenError;
fn gen_bindings(header:&str,output:&str,out_path: &Path){
    let full_out_path = out_path.join(output);
    bindgen::Builder::default()
    .header(header)
    .clang_args([
        "-nostdinc++", // No libc++ is used
        "-nostdlib",
        "-nostartfiles", 


        "-nostdlib",
        "-I${workspaceFolder}/include"
        ]) 
        .use_core()
        // .clang_arg(" -nostdinc")  // Not yet...
        // .allowlist_file(arg)
        .generate()
        
        .unwrap_or_else(|e| panic!("Failed to gen bindings for `{}` with bindgen error msg: {}", header,e))
        
        .write_to_file(&full_out_path)
        
        .unwrap_or_else(|e|{
            use std::io::ErrorKind::*;
            let k = e.kind();
            match k{
                ReadOnlyFilesystem => {panic!("Attempted to write to a readonly filesystem")},
                IsADirectory       => {panic!("Attempted to write bindings to directory {}",&full_out_path.display())},
                StorageFull        => {panic!("Failed as file system is too full to write bindings")},
                TooManyLinks       => {panic!("Too many hard links for {}",&full_out_path.display())}, // I'm not sure if this can even occur
                InvalidFilename    => {panic!("Invalid file name or exceeded length {}",&full_out_path.display())},
                QuotaExceeded      => {panic!("Writing error due to exceeded quota for {}",&full_out_path.display())},
                FilesystemLoop     => {panic!("Symlink loop or long chain of symlinks occurred somewhere in the path of {}"
                                            ,&full_out_path.display())}, // Maybe implement on where the loop is occurring, but that might be out of scope
                Other              => {panic!("Writing Error, Unspecified Kind (non-std defined): {}",e)},
                _                  => {panic!("Writing Error, un-treated kind `{}` with error value {}",k,e)}
            }
        });
}