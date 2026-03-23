#!/bin/env bash

# out=$(mktemp)
out=./test.elf
clang -nostartfiles  -nodefaultlibs -nostdlib ./tests/compile-test.c ./target/debug/librs_libc.so ./target/debug/libcrt0.a -g -isystem "${BASH_SOURCE[0]}/../include" -o "$out" 
if [[ "$?" -eq "0" ]]; then
    echo "Compile Successful!"
else
    echo "Something went wrong!" >&2
    exit -1
fi
"$out"
echo "Test program exited with `$?`"
# rm -r $out