
.PHONY: build
build:
	cargo build


.PHONY: run
run: build
	target/debug/rust-prac


.PHONY: test
test: 
	cargo test