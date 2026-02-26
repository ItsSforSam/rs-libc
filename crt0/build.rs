#![allow(missing_docs, reason ="build script")]

use std::ffi::OsString;
use std::env::{var as evar, var_os as evar_os,VarError};
fn main(){
    println!("cargo::rerun-if-changed=src");
    // std::pr
    let out_dir = std::path::PathBuf::from(evar_os("OUT_DIR").unwrap());
    let out_file = out_dir.with_file_name("crt0-asm.o");
    let target_arch = evar("CARGO_CFG_TARGET_ARCH").unwrap();
    // let linker = std::path::PathBuf::from(std::env::var_os("RUSTC_LINKER").unwrap_or("/usr/bin/ldd".into()));
    let assembler = std::path::PathBuf::from(std::env::var_os("HOST_ASSEMBLER").unwrap_or("/usr/bin/as".into()));
    let assembler_flags = evar_os("ASSEMBLER_FLAGS").unwrap_or("".into());
    
    let mut assumed_flags:Vec<&str> = Vec::new();
    let debug = evar("DEBUG");

    match debug.as_ref(){
        Ok(v) => {
            let v:&str = v;
            match v {
                "true" => {
                    assumed_flags.push("--gen-debug");
                    assumed_flags.push("--gstabs");
                },
                "false" => {/* Does nothing */},
                other => unreachable!("DEBUG environment variable not true or false, it's `{other}`")
            }
        },
        Err(VarError::NotPresent) => unreachable!("DEBUG environment variable doesn't exist"),
        Err(VarError::NotUnicode(s)) => unreachable!("DEBUG environment not UTF-8??? The value of `{s:?}`")
    }
    let mut asm_path = std::ffi::OsString::from("src/arch/");
    asm_path.push(target_arch);
    asm_path.push(".S");
    println!("Asm file path: {:?}",asm_path);


    let mut binding = std::process::Command::new(&assembler);
    let child = binding
    .arg(asm_path)
    .args(["-Wall","--info"])
    .args(assumed_flags)
    .args([assembler_flags])
    .args([OsString::from("-o"),out_file.clone().into()])
    // .stderr(cfg)
    .stdin(std::process::Stdio::null());
    
    println!("{:?}",&child);
    let mut child = child.spawn()
    .unwrap_or_else(|e|{
        panic!("{}",&format!("Failed to spawn assembler process `{:?}` with error {}",&assembler,e))
    }
    );
    let exit_code = child.wait().unwrap().code().expect("Assembler failed due to signal");
    assert!(exit_code != 0,"Assembler failed with exit code {exit_code}");
    // unimplemented!()
    println!("cargo::rustc-link-lib={}",out_file.display());
}