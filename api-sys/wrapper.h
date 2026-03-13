#define __RSLIBC_BUILD_BOOTSTRAP 1
#include <sys/stat.h>
#include <linux/types.h>
#include <linux/fcntl.h>
#include <asm/unistd.h>
#include <linux/errno.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <bits/syscall.h>
#include <signal.h>
#include <elf.h>

// @TODO: for some reason this isn't being included properly in bits/syscall, even tho
// on my system it has mmap2 supported.
// This should fallback to normal mmap if it isn't supported properly
// but Rust doesn't allow any equivent to `#if defined`
#ifndef SYS_mmap2
#define __mmap2_sys 192 // What the value is when supported
#else
#define __mmap2_sys SYS_mmap2
#endif