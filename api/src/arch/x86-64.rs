
// This simply lists the ways for the syscall as the man page doesn't show the quicker instruction of syscall
// As the man page of syscall(2) which shows the registers for `int $0x8` for some reason
// https://en.wikibooks.org/wiki/X86_Assembly/Interfacing_with_Linux#Via_dedicated_system_call_invocation_instruction
use core::arch::asm;
use core::ffi::c_long as long;

/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall0(mut sc:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            out("rcx") _,
            out("r11")  _,
            options(nostack)
        }

    }
    sc 
}

/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall1(mut sc:long, p1:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}

/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall2(mut sc:long, p1:long, p2:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            in("rsi")    p2,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}
/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall3(mut sc:long, p1:long, p2:long,p3:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            in("rsi")    p2,
            in("rdx")    p3,
            // in("r10")    p4,
            // in("r8")     p5,
            // in("r9")     p6,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}
/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall4(mut sc:long, p1:long, p2:long,p3:long,p4:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            in("rsi")    p2,
            in("rdx")    p3,
            in("r10")    p4,
            // in("r8")     p5,
            // in("r9")     p6,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}
/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall5(mut sc:long, p1:long, p2:long,p3:long,p4:long,p5:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            in("rsi")    p2,
            in("rdx")    p3,
            in("r10")    p4,
            in("r8")     p5,
            // in("r9")     p6,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}
/// # Safety
/// Performing a raw syscall with malformed prams can cause undefined behaver
#[inline(always)]
pub unsafe fn syscall6(mut sc:long, p1:long, p2:long,p3:long,p4:long,p5:long,p6:long) -> long{
    // SAFETY: caller guarantees that no undefined behaver occurs
    unsafe {
        
        asm!{
            "syscall",
            inout("rax") sc,
            in("rdi")    p1,
            in("rsi")    p2,
            in("rdx")    p3,
            in("r10")    p4,
            in("r8")     p5,
            in("r9")     p6,
            out("rcx")   _,
            out("r11")   _,
            options(nostack)
        }

    }
    sc
}