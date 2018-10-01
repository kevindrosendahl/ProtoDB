.PHONY: all
all:
	cargo +beta build

.PHONY: run
run:
	cargo +beta run

.PHONY: fmt
fmt:
	cargo +beta fmt

.PHONY: clippy
clippy:
	cargo +beta clippy
