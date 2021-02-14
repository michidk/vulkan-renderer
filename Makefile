.PHONY: run build shaders check test clippy clippy-hack fmt lint cic cicl clean

# run and compile
run:
	cargo +nightly run

build:
	cargo +nightly build

shaders:
	mkdir -p ./assets/shaders; ve_shader ./shaders/* -o ./assets/shaders

# test and lint
check:
	cargo +nightly check
	cargo +nightly check -p math

test:
	cargo +nightly test
	cargo +nightly test -p math

clippy:
	cargo +nightly clippy -- --tests -D warnings

clippy-hack:
	# hack to update files so that clippy/cargo does not use cached versions
	find -name "*.rs" -not -path "./target/*" -exec touch "{}" +
	cargo +nightly clippy -- --tests -D warnings

fmt:
	cargo +nightly fmt --all -- --check

lint: fmt clippy

# utility
## can i commit?
cic: test fmt clippy
	cargo +nightly clippy -- -D warnings

## cic hack
cicl: test fmt clippy-hack

clean:
	cargo clean
