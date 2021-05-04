release:
	cargo build --release

dev: format lint
	make test &
	make doc
	cargo build

d:
	cargo watch -c -s 'echo "" && make dev'

format:
	cargo fmt

lint:
	cargo clippy --all-targets

test:
	cargo test --all-targets

doc:
	cargo doc
