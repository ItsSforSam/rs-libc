#ifndef _SYS_MMAN_H

#include <asm/mman.h>
#ifdef __linux__
    #if defined(__has_include) && !__has_include(<linux/mman.h>)
        #error "Error cannot get linux api headers. Ensure they are installed on your system"
    #else /*If cannot determine if it can be included or can, then just try */
        #include <linux/mman.h>
    #endif /*has_include(<linux/mman.h>)*/
#endif /* __linux__ */

#define PROT_READ	    0x1		/* Can be read  */
#define PROT_WRITE	    0x2		/* Can be written */
#define PROT_EXEC	    0x4		/* Can be executed */
#define PROT_NONE	    0x0		/* Can't be accessed  */
#define PROT_GROWSDOWN	0x01000000	/* Extend change to start of growsdown vma (mprotect only).  */
#define PROT_GROWSUP	0x02000000	


#define MAP_32BIT       0x40            /* Only give out 32-bit addresses.  */
#define MAP_ABOVE4G     0x80            /* Only map above 4GB.  */
/* Sharing types (must choose one and only one of these).  */
#define MAP_SHARED	0x01		/* Share changes.  */
#define MAP_PRIVATE	0x02		/* Changes are private.  */
#define MAP_SHARED_VALIDATE	0x03	/* Shares but errors on unknown flags  */
#define MAP_TYPE	0x0f		/* Mask for type of mapping.  */
/* Other flags.  */
#define MAP_FIXED	0x10		/* Don't use addr as hint. Use the address exact;y*/
#define MAP_FILE	0

// MAP_ANONYMOUS has it not use a file, fd param is ignored
#ifdef __MAP_ANONYMOUS
    #define MAP_ANONYMOUS	__MAP_ANONYMOUS
#else
    #define MAP_ANONYMOUS	0x20		
#endif
#define MAP_ANON	MAP_ANONYMOUS
/* When MAP_HUGETLB is set bits [26:31] encode the log2 of the huge page size.  */
#define MAP_HUGE_SHIFT	26
#define MAP_HUGE_MASK	0x3f



#define MAP_GROWSDOWN   0x00100         /* Stack-like segment.  */
#define MAP_DENYWRITE   0x00800         /* ETXTBSY. Now is ignored  */
#define MAP_EXECUTABLE  0x01000         /* Mark it as an executable.  */
#define MAP_LOCKED      0x02000         /* Lock the mapping.  */
#define MAP_NORESERVE   0x04000         /* Don't check for reservations.  */
#define MAP_POPULATE    0x08000         /* Populate (prefault) pagetables.  */
#define MAP_NONBLOCK    0x10000         /* Do not block on IO.  */
#define MAP_STACK       0x20000         /* Allocation is for a stack.  */
#define MAP_HUGETLB     0x40000         /* Create huge page mapping.  */
#define MAP_SYNC        0x80000         /* Perform synchronous page faults for the mapping.  */
#define MAP_FIXED_NOREPLACE 0x100000    /* MAP_FIXED but don't allow overlaps  */



#endif /*_SYS_MMAN_H*/