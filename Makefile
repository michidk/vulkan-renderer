.PHONY: all

run:
	cargo +nightly run

check:
	cargo +nightly check

test:
	cargo +nightly test

lint:
	cargo +nightly fmt --all -- --check
	cargo +nightly clippy -- -D warnings

canICommit:
	make check
	make test
	make lint

clean:
	cargo clean
