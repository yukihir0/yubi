.PHONY: development
development:
	RUST_LOG=info cargo run -- spec.yml

.PHONY: debug
debug:
	RUST_LOG=debug cargo run -- spec.yml

.PHONY: test
test:
	cargo test

.PHONY: test_watch
test_watch:
	cargo watch -x test

.PHONY: audit
audit:
	cargo audit

.PHONY: build
build:
	cargo build --release
