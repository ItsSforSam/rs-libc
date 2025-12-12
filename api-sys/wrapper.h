#define __RSLIBC_BUILD_BOOTSTRAP 1
#include <asm/unistd.h>
#include <linux/errno.h>
#include <sys/mman.h>
#include <sys/syscall.h>
#include <bits/syscall.h>
#include <signal.h>
#include <sys/stat.h>