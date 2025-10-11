# CHIP-8 Emulator Makefile

# Cross-platform SDL2 path setup
ifeq ($(OS),Windows_NT)
    # Windows - usually no special setup needed
    # SDL2 should be available via vcpkg or MSYS2
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Darwin)
        # macOS - Homebrew paths
        export PKG_CONFIG_PATH := /opt/homebrew/lib/pkgconfig:$(PKG_CONFIG_PATH)
        export LIBRARY_PATH := /opt/homebrew/lib:$(LIBRARY_PATH)
    endif
    # Linux - usually no special setup needed
endif

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
