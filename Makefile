.PHONY: check test

check:
	cargo install cargo-hack
	cargo hack check --feature-powerset

test:
	cargo install cargo-hack
	cargo hack test --feature-powerset