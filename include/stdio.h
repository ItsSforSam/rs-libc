#ifndef _STDIO_H
#define _STDIO_H


// #include <stddef.h>
#ifdef __RSLIBC_BUILD_BOOTSTRAP
extern size_t write(int fd,const void* buffer, size_t nbytes);

/* Opaque struct */
typedef struct FILE;

#endif

#endif