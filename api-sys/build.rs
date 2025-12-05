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
    .clang_args([
        "-nostdinc++", // No libc++ is used
        "-isystem", "../include", // Treat these as system files
        "-nostdlibinc", // Don't search standard system includes but keep searching compiler includes
        "-idirafter", &get_includes() // add the normal
        
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


static INCLUDES_ERR:&str = "An invalid"

fn get_includes() -> String{
    use std::env::VarError;
    
    match std::env::var("CPATH") {
        Ok(v) => {
                            let p = std::path::
                            let e = std::fs::exists(&v);

                        },
        #[cfg(unix)]
        Err(VarError::NotPresent) => String::from("/usr/include"),
        #[cfg(unix)]
        Err(VarError::NotUnicode(v)) => panic!("Non UTF-8 value from `CPATH` (bindgen requires valid UTF-8). Value if {}", v.display()),
        #[cfg(not(unix))]
        Err(_) => unimplemented!("Linux is the only supported OS currently")
    }
}

fn handle_includes_err(err:std::io::Error){}
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