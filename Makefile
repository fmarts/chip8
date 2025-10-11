# CHIP-8 Emulator Makefile

.PHONY: build run clean test

build:
	cargo build --release

run:
	cargo run --release $(filter-out $@,$(MAKECMDGOALS))

clean:
	cargo clean

test:
	cargo test

# Usage: make run path/to/game
