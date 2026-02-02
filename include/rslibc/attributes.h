/*
    Defines macros which are for compiler specific attributes

    @internal
*/

#ifndef __rslibc_internal_attributes_h
#define __rslibc_internal_attributes_h

#if defined(__GCC__) | defined(__CLANG__)
#define __NoReturn __attribute__((__noreturn))
#else
#define __NoReturn /*Expands to nothing but is used for optimizations if compiler supports it*/
#endif /*!defined(__GCC__ | __CLANG__)*/

#endif /* __rslibc_internal_attributes_h include guard*/