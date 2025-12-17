
#ifndef _ERRNO_H
#define _ERRNO_H

#include <sys/errno.h>
/*
  We generate consts and non-libc functions to use in rust.
  To avoid having functions and the like to be attempted to be
  linked found with sys crate when it doesn't exit 
*/
#ifndef __RSLIBC_BUILD_BOOTSTRAP

    int* __get_errno_ptr();
    #define errno *__get_errno_ptr()

#endif /*__RSLIBC_BUILD_BOOTSTRAP */
#endif