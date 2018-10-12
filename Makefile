.PHONY: all
all:
	cargo build
	@$(MAKE) fmt

.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: clippy
clippy:
	cargo clippy
