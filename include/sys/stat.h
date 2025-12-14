/*
This ports linux headers which is under GPL-2.0 WITH linux-syscall-note

https://github.com/torvalds/linux/blob/d358e5254674b70f34c847715ca509e46eb81e6f/arch/mips/include/uapi/asm/fcntl.h#L17
*/

#ifndef _SYS_STAT_H
#define _SYS_STAT_H
#include <asm-generic/fcntl.h>
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
#endif /*_SYS_STAT_H */