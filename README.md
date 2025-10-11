# CHIP-8 Emulator

A modern CHIP-8 emulator written in Rust

## Features

- Complete CHIP-8 instruction set implementation
- Fast graphics rendering
- Cross-platform compatibility
- Proper error handling

## Requirements

- Rust 1.70+ (2021 edition)

## Building and Running

### Using Makefile (Recommended)

**Unix/Linux/macOS:**
```bash
make run <path_to_rom>
```

**Windows:**
```cmd
# Install make via chocolatey: choco install make
make run <path_to_rom>
```

### Manual Build
```bash
# Build the emulator
cargo build --release

# Run the emulator
cargo run --release <path_to_rom>
```

## Controls

The emulator uses the following key mapping for CHIP-8's hex keypad:

```
1 2 3 C
4 5 6 D  
7 8 9 E
A 0 B F
```

Mapped to:

```
1 2 3 4
Q W E R
A S D F
Z X C V
```

## Resources
* [Cowgod's Chip8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
