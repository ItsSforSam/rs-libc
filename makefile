#https://doc.rust-lang.org/rustc/targets/custom.html
RUST_TARGET_PATH := "./custom-targets/"
export RUST_TARGET_PATH
.PHONEY: clean test crt0 rs-libc default

default: crt0 rs-libc

crt0: target/debug/libcrt0.a

rs-libc: target/debug/librs_libc.so

target/debug/libcrt0.a: crt0/*
	cargo build -p crt0

target/debug/librs_libc.so: src/*

# @TODO: have custom targets work
# --target="./custom-targets/x86_64-unknown-linux.json" -Zbuild-std=core,alloc

#   Have to specifiy the TLS model due to how #[thread_local] attribute works
# 	https://doc.rust-lang.org/beta/unstable-book/compiler-flags/tls-model.html
	RUSTFLAGS="-Ztls-model=initial-exec" cargo build -p rs-libc 
clean:
	cargo clean

test: crt0 rs-libc
	@echo "Testing if successful compile"
	@tests/compile-test.sh