#https://doc.rust-lang.org/rustc/targets/custom.html
RUST_TARGET_PATH := "./custom-targets/"
export RUST_TARGET_PATH
OUT_DIR ?= ./bin
CARGO_TARGET_DIR ?= ./target
export CARGO_TARGET_DIR
# Have the release debug be pulled away
CARGO_BIN_LOCATION := $(CARGO_TARGET_DIR)/release

.PHONEY: clean test crt0 rs-libc rs-libc-shared rs-libc-static default check check-crt0 check-main

default: crt0 rs-libc

crt0: $(CARGO_BIN_LOCATION)/libcrt0.a

rs-libc: rs-libc-shared rs-libc-static
rs-libc-shared: $(CARGO_BIN_LOCATION)/librs_libc.so

rs-libc-static: $(CARGO_BIN_LOCATION)/librs_libc.a

$(CARGO_BIN_LOCATION)/libcrt0.a: $(wildcard crt0/*)
# We do this to compile
	cargo build -p crt0 -Zbuild-std=core --release

$(CARGO_BIN_LOCATION)/librs_libc.so: $(wildcard src/* )

# @TODO: have custom targets work
# --target="./custom-targets/x86_64-unknown-linux.json" -Zbuild-std=core,alloc

#   Have to specifiy the TLS model due to how #[thread_local] attribute works
# 	https://doc.rust-lang.org/beta/unstable-book/compiler-flags/tls-model.html
	RUSTFLAGS="-Ztls-model=initial-exec" cargo build -p rs-libc --release

$(CARGO_BIN_LOCATION)/librs_libc.a: target/release/librs_libc.so
# TODO have this only compile static
# 	cargo build -p rs-libc --release

clean:
	cargo clean
# 	We Want to 
	-rm -rf $(OUT_DIR) --one-file-system --preserve-root

test: crt0 rs-libc-static
	@echo "Testing if successful compile"
	@tests/compile-test.sh
