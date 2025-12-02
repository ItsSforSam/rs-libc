#ifndef _SYS_ERRNO_H
#define _SYS_ERRNO_H

#include <linux/errno.h> /* Just get it from the source */

/* For backporting as the newer headers don't contain these */
#ifndef ENOTSUP
    #define ENOTSUP EOPNOTSUPP
#endif

#ifndef ECANCELED
    #define ECANCELED        125
#endif

#ifndef EOWNERDEAD
    #define EOWNERDEAD       130
#endif

#ifndef ENOTRECOVERABLE
    #define ENOTRECOVERABLE  131
#endif

#ifndef ERFKILL
    #define ERFKILL          132
#endif

#ifndef EHWPOISON
    #define EHWPOISON        133
#endif

#endif /* _SYS_ERRNO_H */