#!/bin/env bash

# out=$(mktemp)
out=./test.elf
clang -nostartfiles  -nodefaultlibs -nostdlib ./tests/compile-test.c ./target/release/libcrt0.a  ./target/release/librs_libc.a -g -I"${BASH_SOURCE[0]}/../include" -o "$out" -static -lgcc -Wall 
if [[ "$?" -eq "0" ]]; then
    echo "Compile Successful!"
else
    echo "Something went wrong!" >&2
    exit -1
fi
$out
exit=$?
echo "Test program exited with \`$exit\`"
exit $exit # exit with it so it can be programmatically determined it failed
# rm -r $out