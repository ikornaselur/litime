lint:
	@touch src/*.rs
	@cargo +nightly clippy

build_osx_release:
	mkdir -p artifacts
	cargo build --release
	mv target/release/litime artifacts/litime-osx
	strip artifacts/litime-osx

build_linux_release:
	mkdir -p artifacts
	docker run --rm -it -v "$(PWD)":/home/rust/src omnijar/rust:linux-musl /bin/bash -c "cargo build --release && strip target/x86_64-unknown-linux-musl/release/litime"
	mv target/x86_64-unknown-linux-musl/release/litime artifacts/litime-linux

build_release: build_osx_release build_linux_release
