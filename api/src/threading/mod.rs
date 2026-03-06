//! The threading model
//! 
//! Allows multithreading and support for pthreads


/// Access Thread local storage
// https://doc.rust-lang.org/beta/unstable-book/compiler-flags/tls-model.html
// We use initial-exec currently there was linker errors due to the linker not being able to find
// "__tls_get_addr", but after setting that the error goes away, and I cannot find what function it expects
// as removing this doesn't cause any warning, so what is it doing to access TLS?
#[unsafe(no_mangle)]
#[doc(hidden)]
pub unsafe extern "C" fn __tls_get_addr(_v:usize){
    todo!("No implementation of TLS is available right now")
    
}