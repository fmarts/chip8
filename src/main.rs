#![allow(unused_variables)]

mod chip8;
mod screen;
mod instruction;

use clap::Parser;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
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

    let ctx = sdl2::init().expect("Failed to initialize SDL2");
    let mut chip8 = Chip8::new(&ctx);
    let mut events = ctx.event_pump().expect("Failed to get event pump");

    chip8.load_rom(&args.file);

    // CHIP-8 timing: Run multiple cycles per frame for better speed
    let mut last_tick = Instant::now();
    let frame_duration = Duration::from_millis(16); // ~60Hz frame rate

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    match key {
                        Keycode::Num1 => chip8.press(0x1),
                        Keycode::Num2 => chip8.press(0x2),
                        Keycode::Num3 => chip8.press(0x3),
                        Keycode::Num4 => chip8.press(0xc),
                        Keycode::Q    => chip8.press(0x4),
                        Keycode::W    => chip8.press(0x5),
                        Keycode::E    => chip8.press(0x6),
                        Keycode::R    => chip8.press(0xd),
                        Keycode::A    => chip8.press(0x7),
                        Keycode::S    => chip8.press(0x8),
                        Keycode::D    => chip8.press(0x9),
                        Keycode::Z    => chip8.press(0xa),
                        Keycode::X    => chip8.press(0x0),
                        Keycode::C    => chip8.press(0xb),
                        Keycode::V    => chip8.press(0xf),
                        _             => {},
                    }
                },
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    match key {
                        Keycode::Num1 => chip8.release(0x1),
                        Keycode::Num2 => chip8.release(0x2),
                        Keycode::Num3 => chip8.release(0x3),
                        Keycode::Num4 => chip8.release(0xc),
                        Keycode::Q    => chip8.release(0x4),
                        Keycode::W    => chip8.release(0x5),
                        Keycode::E    => chip8.release(0x6),
                        Keycode::R    => chip8.release(0xd),
                        Keycode::A    => chip8.release(0x7),
                        Keycode::S    => chip8.release(0x8),
                        Keycode::D    => chip8.release(0x9),
                        Keycode::Z    => chip8.release(0xa),
                        Keycode::X    => chip8.release(0x0),
                        Keycode::C    => chip8.release(0xb),
                        Keycode::V    => chip8.release(0xf),
                        _             => {},
                    }
                },
                _                 => {},
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