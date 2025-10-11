#![allow(unused_variables)]

mod chip8;
mod screen;
mod instruction;

use clap::Parser;
use minifb::Key;
use std::time::{Duration, Instant};
use chip8::Chip8;

#[derive(Parser)]
#[command(name = "chip8")]
#[command(about = "A CHIP-8 emulator")]
#[command(version)]
struct Args {
    /// ROM file to load
    file: String,
}

fn main() {
    let args = Args::parse();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&args.file);

    // CHIP-8 timing: Run multiple cycles per frame for better speed
    let mut last_tick = Instant::now();
    let frame_duration = Duration::from_millis(16); // ~60Hz frame rate

    while chip8.screen.is_open() {
        // Handle input
        let keys = chip8.screen.get_keys();
        chip8.reset_keys();
        
        for key in keys {
            match key {
                Key::Key1 => chip8.press(0x1),
                Key::Key2 => chip8.press(0x2),
                Key::Key3 => chip8.press(0x3),
                Key::Key4 => chip8.press(0xc),
                Key::Q    => chip8.press(0x4),
                Key::W    => chip8.press(0x5),
                Key::E    => chip8.press(0x6),
                Key::R    => chip8.press(0xd),
                Key::A    => chip8.press(0x7),
                Key::S    => chip8.press(0x8),
                Key::D    => chip8.press(0x9),
                Key::Z    => chip8.press(0xa),
                Key::X    => chip8.press(0x0),
                Key::C    => chip8.press(0xb),
                Key::V    => chip8.press(0xf),
                _         => {},
            }
        }

        // Run multiple CHIP-8 cycles per frame for better speed
        for _ in 0..10 {
            chip8.run();
        }

        // Maintain consistent frame rate
        let now = Instant::now();
        if now.duration_since(last_tick) >= frame_duration {
            last_tick = now;
        } else {
            std::thread::sleep(frame_duration - now.duration_since(last_tick));
            last_tick = Instant::now();
        }
    }

    println!("Exiting..");
}