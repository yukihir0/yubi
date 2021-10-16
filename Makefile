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

.PHONY: test_coverage
test_coverage:
	./script/coverage.sh

.PHONY: audit
audit:
	cargo audit

.PHONY: build
build:
	cargo build --release

.PHONY: renovate
renovate:
	docker run \
		--env RENOVATE_TOKEN=$$RENOVATE_TOKEN \
		--env RENOVATE_CONFIG_FILE=/github-action/renovate.json \
		--volume $$PWD/renovate.json:/github-action/renovate.json \
		--volume /var/run/docker.sock:/var/run/docker.sock.raw \
		--volume /tmp:/tmp \
		--rm renovate/renovate:27.31.4-slim \

.PHONY: renovate_dry_run
renovate_dry_run:
	docker run \
		--env RENOVATE_TOKEN=$$RENOVATE_TOKEN \
		--env RENOVATE_CONFIG_FILE=/github-action/renovate.json \
		--volume $$PWD/renovate.json:/github-action/renovate.json \
		--volume /var/run/docker.sock:/var/run/docker.sock.raw \
		--volume /tmp:/tmp \
		--rm renovate/renovate:27.31.4-slim \
		--dry-run
