#![no_std]

unsafe extern "C" {
    // Defined in panic handler
    #[link_name = "__rslibc_abort_internal"]
    pub safe fn abort() -> !; 

}