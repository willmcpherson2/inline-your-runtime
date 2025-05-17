ARCH=x86_64-unknown-linux-gnu

.PHONY: debug
debug:
	$(MAKE) build DIR=debug PROFILE=dev

.PHONY: release
release:
	$(MAKE) build DIR=release PROFILE=release

.PHONY: build
build:
	RUSTFLAGS="--emit=llvm-bc" cargo build \
		--package rts \
		--profile $(PROFILE) \
		--target $(ARCH) \
		-Z build-std=core,compiler_builtins,alloc

	llvm-link \
		target/$(ARCH)/$(DIR)/deps/core-*.bc \
		target/$(ARCH)/$(DIR)/deps/compiler_builtins-*.bc \
		target/$(ARCH)/$(DIR)/deps/alloc-*.bc \
		target/$(ARCH)/$(DIR)/deps/libc-*.bc \
		target/$(ARCH)/$(DIR)/deps/rts-*.bc \
		-o target/rts.bc

	opt \
		--internalize-public-api-list="new_foo,sum_foo,free_foo" \
		--passes="internalize,globaldce" \
		target/rts.bc \
		-o target/rts.bc
