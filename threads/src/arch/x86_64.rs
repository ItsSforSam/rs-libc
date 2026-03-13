use core::ffi;
//https://www.akkadia.org/drepper/tls.pdf
/// Used to compute to get a given thread local value in a given thread
// This depends on the architecture
#[unsafe(no_mangle)]
pub extern "C" fn __tls_get_addr(ti: *mut tls_index)->*mut ffi::c_void{
    todo!()
}

#[repr(C)]
#[derive(Debug)]
pub struct tls_index{
    module: ffi::c_ulong,
    offset: ffi::c_ulong
}

/// Get a thread pointer
/// 
/// # SAFETY
/// If offset is invalid it will read undefined memory
#[inline]
pub(crate) unsafe fn get_tp(offset:usize)->*mut ffi::c_void{
    let tp:*mut ffi::c_void;
    // SAFETY: Valid Asm
    unsafe {core::arch::asm!{
        
        "mov %fs:[{}],{}",
        in(reg) offset,
        out(reg) tp,

        options(att_syntax, nostack, readonly,preserves_flags)  
    }}
    return tp;
}
