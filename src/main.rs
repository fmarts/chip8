#![allow(unused_variables)]

mod chip8;
mod screen;
mod instruction;

extern crate rustc_serialize;
extern crate docopt;
extern crate sdl2;
extern crate num;
extern crate rand;
#[macro_use] extern crate enum_primitive as ep;

use docopt::Docopt;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use chip8::Chip8;

const USAGE: &'static str = "
Chip8 Emulator

Usage:
    chip8 <file>
    chip8 (-h | --help)
    chip8 (-v | --version)

Options:
    -h --help       Show this screen
    -v --version    Show version
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_file: String,
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let mut chip8 = Chip8::new(&ctx);
    let mut events = ctx.event_pump().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    chip8.load_rom(&args.arg_file);

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    chip8.reset_keys();
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
                        Keycode::Z    => chip8.press(0x0),
                        Keycode::X    => chip8.press(0xb),
                        Keycode::F    => chip8.press(0xf),
                        _             => {},
                    }
                },
                _                 => {},
            }
        }

        chip8.run();
    }

    println!("Exiting..");
}
