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
use sdl2::event::{Event};
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
                Event::Quit{..}     => break 'main,
                _                   => continue
            }
        }

        chip8.run();
        //println!("{:?}", chip8); 
        //stdin().read_line(&mut buf);
    }

    println!("Exiting..");
}
