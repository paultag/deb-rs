SUPPORTED_FEATURES = \
	serde \
	chrono \
	hex \
	tokio \
	sequoia \
	serde,hex \
	serde,tokio \
	serde,chrono \
	serde,chrono,hex \
	serde,chrono,tokio \
	serde,hex,tokio \

all: build test check TODO

check: test check-lint

test: test-no-default $(patsubst %,test-%,$(SUPPORTED_FEATURES)) docs
	cargo test
	$(shell cd examples && cargo test)

test-no-default:
	cargo test --no-default-features --features ''

test-%:
	cargo test --no-default-features --features $(patsubst test-%,%,$@)

check-lint:
	cargo clippy --all-features
	cd benches  && cargo clippy
	cd fuzz     && cargo clippy

docs:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features --no-deps

clean:
	rm -rvf \
		target/ \
		benches/target/ \
		examples/target/

.PHONY: docs build check test TODO
