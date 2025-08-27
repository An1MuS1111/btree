
.PHONY: build
build:
	cargo build


.PHONY: run
run: build
	target/debug/btree


.PHONY: test
test: 
	cargo test