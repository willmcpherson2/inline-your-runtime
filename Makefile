.PHONY: rts
rts:
	RUSTFLAGS="--emit=llvm-bc" cargo build -p rts -Z build-std=core,alloc -Z panic-abort-tests --target x86_64-unknown-linux-gnu --release
	llvm-link \
		--only-needed \
		target/x86_64-unknown-linux-gnu/release/deps/rts-*.bc \
		target/x86_64-unknown-linux-gnu/release/deps/alloc-*.bc \
		target/x86_64-unknown-linux-gnu/release/deps/core-*.bc \
		-o target/rts.bc
