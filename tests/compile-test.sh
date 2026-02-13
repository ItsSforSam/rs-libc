#!/bin/env bash

# out=$(mktemp)
out=./test.elf
clang -nostartfiles  -nodefaultlibs -nostdlib ./tests/compile-test.c ./target/debug/librs_libc.so ./target/debug/libcrt0.a -isystem ./include -o "$out"
"$out"
echo $?
# rm -r $out