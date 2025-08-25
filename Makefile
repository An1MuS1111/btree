
.PHONY: build
build:
	cargo build


.PHONY: run
run: build
	target/debug/rust-proj