#![feature(io_error_more)]
use std::path::{Path, PathBuf};
use std::env;

// extern crate bindgen;
// fn is_linux(){

// }

fn main(){
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=wrapper.h");
    let out_path:PathBuf = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR env not set, are you using cargo?"));
    
    /*
    api:PathBuf;
    let api:PathBuf = PathBuf::from(env::var_os("OS_API").unwrap_or("/usr/include/linux/".into()));
    */
    // let bindings = bindgen::Builder::default().header("wrapper.h").generate().expect("Failed to gen bindings");
    // let unistd = &out_path.join("unistd.rs");
    // // println!("{}", unistd.display());
    // bindings.write_to_file(unistd).expect("Failed to write bindings");
    
    // bindgen::Builder::default()
    // .header("../include/sys/syscall.h")
    // .generate().expect("Failed to gen bindings for syscall.h")
    // .write_to_file(out_path.join("syscall.rs")).unwrap();
    gen_bindings("wrapper.h", "unistd.rs", &out_path);
}
// use bindgen::BindgenError;
fn gen_bindings(header:&str,output:&str,out_path: &Path){
    let full_out_path = out_path.join(output);
    bindgen::Builder::default()
    .header(header)
    .clang_arg("-D__RSLIBC_BUILD_GEN=1") // Has the headers not include functions, only really constants and macros
    .clang_arg("-nostdinc++") // No libc++ is used
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

// // fn gen_headers(f:&Path) -> io::Result<()>{
// //     let mut dir:Vec<&Path> = Vec::new();
// //     for b in fs::read_dir(f)?{
// //         let file = b?;
// //         let meta = file.metadata()?;
// //         if meta.is_dir() {
// //             dir.push(&file.path());
// //             continue;
// //         } else if !meta.file_type().is_file() {
// //             unreachable!(); // There shouldn't be these irregular files within with includes 
// //         }
            
// //         let name = Path::new(&file.file_name()).file_stem();
// //         

// //     }
// //     Ok(())
// // }