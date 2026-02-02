#ifndef _UNISTD_H
#define _UNISTD_H
#include <rslibc/attributes.h>
#ifndef __RSLIBC_BUILD_BOOTSTRAP

extern void _exit(int status) __NoReturn;
#define _Exit(status) _exit(status)
#endif /*__RSLIBC_BUILD_BOOTSTRAP*/
#endif /* _H_UNISTD */