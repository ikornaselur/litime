lint:
	@touch src/*.rs
	@cargo +nightly clippy
