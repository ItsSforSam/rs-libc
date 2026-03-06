use core::mem::{self, MaybeUninit};
use core::ffi::{c_int, c_void};

type SAHandler = Option<extern "C" fn(c_int)>; 
type SASigaction = Option<extern "C" fn(c_int,*mut siginfo,*mut c_void)>;
/// Representation of the sigaction struct
#[repr(C)]
#[derive(Debug)]
pub struct sigaction{
    // The man page lists that sa_handler and sa_action
    // are sometimes a union and one must not set both
    // The actual kernel includes has them be a union only
    // if __i386__ is defined, which is only defined on x86_32 
    #[cfg(target_arch = "x86")]
    sa_handler:MaybeUninit<UnionSigAction>,
    #[cfg(not(target_arch = "x86"))]
    sa_handler:MaybeUninit<SAHandler>,
    #[cfg(not(target_arch = "x86"))]
    sa_sigaction:MaybeUninit<SASigaction>,
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>
}
#[cfg(target_arch = "x86")]
#[repr(C)]
union UnionSigAction{
    sa_handler:SAHandler,
    sa_sigaction:SASigaction

}


impl sigaction{
    pub fn new(action:SigActionHandler, flags:c_int,mask:sigset_t)->sigaction{
        
        let mut r = sigaction {
            
            sa_handler: MaybeUninit::zeroed(),
            #[cfg(not(target_arch = "x86"))]
            sa_sigaction: MaybeUninit::zeroed(),
            sa_mask: mask,
            sa_flags: flags,
            sa_restorer: None
        };
        match action{
            SigActionHandler::Default => {
                r.sa_handler.write(None); // SIG_DFL
            }
            SigActionHandler::Ignore => {
                let ignore  = core::ptr::without_provenance::<SAHandler>(1);
                
                // NOTE: why is attributes on expressions nightly? It should at least work with lint attrs
                #[expect(clippy::missing_transmute_annotations, reason = "Annotation is above, and can vary on platform")]
                {
                // SAFETY: Casting a raw unaligned pointer to MaybeUninit<fn(...)> which will be a valid state
                // as this is passed to the kernel which checks if it's 0 or 1 before deref
                // Rust should not treat this a reference and dereference it
                r.sa_handler = unsafe {mem::transmute(ignore)};

                }
            
            },
            SigActionHandler::Handler(h) => {
                #[cfg(target_arch = "x86")]
                let h = UnionSigAction {sa_handler: h };
                // #[cfg(not(target_arch = "x86"))]
                r.sa_handler.write(h);
            },
            SigActionHandler::Action(h) =>{

                cfg_if::cfg_if!{
                    if #[cfg(target_arch="x86")]{
                        r.sa_handler.write(UnionSigAction {sa_sigaction: h });
                    } else{
                        r.sa_sigaction.write(h);
                    }
                }
                
                
            }
        };

        r
        
    }
}

#[derive(Debug,Default)]
pub enum SigActionHandler {
    // Default signal handler for a given signal
    #[default]
    Default,
    // Ignore the signal
    Ignore,
    Handler(SAHandler),
    Action(SASigaction)

}
#[repr(C)]
#[derive(Debug)]
#[non_exhaustive] // temporary and just because we don't have all the fields
pub struct siginfo{
    /// The signal number
    #[doc(alias = "si_signo")]
    pub signo: c_int,
    #[doc(alias = "si_errno")]
    pub errno: c_int,
    /// The signal code
    #[doc(alias = "si_code")]
    pub code:c_int,
    // copied from
    // https://docs.rs/libc/latest/src/libc/unix/linux_like/linux/gnu/b64/x86_64/mod.rs.html#17-316
    /// The raw bytes of the structure, as some of them are unions or just not defined, for example
    /// Like <code>si_addr</code> (memory location of fault) won't be defined if [`SIGINT`] was what was raised
    /// 
    // [`SIGINT`]: crate::errno::Errno::SIGINT
    _pad: [c_int;29],
    // This part could've been to lock out due to pad being exposed via pub
    _align: [u64;0]
}
#[derive(Debug)]
pub struct sigset_t{
    // copied from libc crate
    #[cfg(target_pointer_width = "32")]
    __val: [u32; 32],
    #[cfg(target_pointer_width = "64")]
    __val: [u64; 16],
}