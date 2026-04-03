/*
This ports linux headers which is under GPL-2.0 WITH linux-syscall-note

https://github.com/torvalds/linux/blob/d358e5254674b70f34c847715ca509e46eb81e6f/arch/mips/include/uapi/asm/fcntl.h#L17
*/

#ifndef _SYS_STAT_H
#define _SYS_STAT_H
#include <asm-generic/fcntl.h>
#include <sys/types.h>
#ifndef O_APPEND
    #define O_APPEND	0x0008
#endif
#ifndef O_DSYNC
    #define O_DSYNC		0x0010	/* used to be O_SYNC, see below */
#endif
#ifndef O_NONBLOCK
    #define O_NONBLOCK	0x0080
#endif
    #ifndef O_CREAT
#define O_CREAT		0x0100	/* not fcntl */
#endif
#ifdef O_TRUNC
    #define O_TRUNC		0x0200	/* not fcntl */
#endif
#ifndef O_EXCL
    #define O_EXCL		0x0400	/* not fcntl */
#endif
#ifndef O_NOCTTY
    #define O_NOCTTY	0x0800	/* not fcntl */
#endif
    #ifndef FASYNC
#define FASYNC		0x1000	/* fcntl, for BSD compatibility */
#endif
#ifndef O_LARGEFILE
    #define O_LARGEFILE	0x2000	/* allow large file opens */
#endif
/*
 * Before Linux 2.6.33 only O_DSYNC semantics were implemented, but using
 * the O_SYNC flag.  We continue to use the existing numerical value
 * for O_DSYNC semantics now, but using the correct symbolic name for it.
 * This new value is used to request true Posix O_SYNC semantics.  It is
 * defined in this strange way to make sure applications compiled against
 * new headers get at least O_DSYNC semantics on older kernels.
 *
 * This has the nice side-effect that we can simply test for O_DSYNC
 * wherever we do not care if O_DSYNC or O_SYNC is used.
 *
 * Note: __O_SYNC must never be used directly.
 */
#ifndef O_SYNC
    #define __O_SYNC	0x4000
    #define O_SYNC		(__O_SYNC|O_DSYNC)
#endif /*O_SYNC*/
#ifndef O_DIRECT
    #define O_DIRECT	0x8000	/* direct disk access hint */
#endif
#ifndef F_GETLK
#define F_GETLK		14
#endif
#ifndef F_SETLK
#define F_SETLK		6
#endif
#ifndef F_SETLKW
#define F_SETLKW	7
#endif
#ifndef F_SETOWN
#define F_SETOWN	24	/*  for sockets. */
#endif
#ifndef F_GETOWN
#define F_GETOWN	23	/*  for sockets. */
#endif

typedef signed long blksize_t

struct stat {
    dev_t      st_dev;      /* ID of device containing file */
    ino_t      st_ino;      /* Inode number */
    mode_t     st_mode;     /* File type and mode */
    nlink_t    st_nlink;    /* Number of hard links */
    uid_t      st_uid;      /* User ID of owner */
    gid_t      st_gid;      /* Group ID of owner */
    dev_t      st_rdev;     /* Device ID (if special file) */
    off_t      st_size;     /* Total size, in bytes */
    blksize_t  st_blksize;  /* Block size for filesystem I/O */
    blkcnt_t   st_blocks;   /* Number of 512 B blocks allocated */

    /* Since POSIX.1-2008, this structure supports nanosecond
        precision for the following timestamp fields.
        For the details before POSIX.1-2008, see VERSIONS.  */

    struct timespec  st_atim;  /* Time of last access */
    struct timespec  st_mtim;  /* Time of last modification */
    struct timespec  st_ctim;  /* Time of last status change */

#define st_atime  st_atim.tv_sec  /* Backward compatibility */
#define st_mtime  st_mtim.tv_sec
#define st_ctime  st_ctim.tv_sec
};
struct stat64
  {
    mode_t st_mode;		/* File mode.  */
    ino64_t st_ino;		/* File serial number.	*/
    dev_t st_dev;		/* Device.  */
    nlink_t st_nlink;		/* Link count.  */

    uid_t st_uid;		/* User ID of the file's owner.	*/
    gid_t st_gid;		/* Group ID of the file's group.*/
    off64_t st_size;		/* Size of file, in bytes.  */

    time_t st_atime;		/* Time of last access.  */
    time_t st_mtime;		/* Time of last modification.  */
    time_t st_ctime;		/* Time of last status change.  */
  };
#endif /*_SYS_STAT_H */