/*
    Used for internal
*/

#ifndef __rslibc_misc_internal
#define __rslibc_misc_internal

#ifdef __GNUC__

/* Define some macros used by GCC's headers like there bits/syscall.h errors if included explicitly and not by there sys/syscall.h*/
/* @TODO: Not to rely on GCC's headers*/
#define _SYSCALL_H 1

#endif

#endif /* __rslibc_misc_internal include guard*/