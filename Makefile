build:
	cargo +nightly build && cargo +nightly build --release

build-release:
	cargo +nightly build --release

build-dev:
	cargo +nightly build
