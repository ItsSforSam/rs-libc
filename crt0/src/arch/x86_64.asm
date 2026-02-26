.att_syntax
# CSpell: disable
.section .text
.weak _start
.global _start
.type _start, @function
_start:
endbr64
# .cfi_undefined %rip # prevent DWARF-based unwinders unwinding further
pop %rdi # argc
mov %rsp, %rsi # argv[]
lea 8(%rsi,%rdi,8),%rdx # then a null, then get the envp
# We want to re-aline the stack. Linux is very good with keeping the ABI compatible
# The issue is dynamic linkers, like musl's ldso which opts to no align the stack when explicitly invoked
# as noted her <https://github.com/ziglang/zig/blob/738d2be9d6b6ef3ff3559130c05159ef53336224/lib/std/start.zig#L240>
xorl %ebp, %ebp # zero stack frame
and $-16, %rsp  # have esp be 16 bits alined
# We call this helper due to us not needing to write EVERYTHING in assembly
call __internal_start_main
# If it returns (it shouldn't) just invoke an invalid instruction
# Set's do hlt right here which is valid but requires ring0 access, which
# shouldn't occur, unless this code is running in kernel space which shouldn't be possible
hlt
#xorl %ebp, %ebp
#movq %rsp, %rdi
#andq $-16, %rsp
#// callq %[posixCallMainAndExit:P]
#callq __internal_start_main