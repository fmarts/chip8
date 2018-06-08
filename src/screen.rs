extern crate sdl2;

use sdl2::render::{Canvas};
use sdl2::{Sdl};

pub struct Screen {
    pub renderer: Canvas<sdl2::video::Window>,
    pub buffer: [u8; 64*32],
}

impl Screen { 
    pub fn new(ctx: &Sdl) -> Screen {
        let video   = ctx.video().unwrap();
        let window  = video.window("chip8", 64*10, 32*10)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        Screen {
            renderer: window.into_canvas().build().unwrap(),
            buffer: [0; 64*32]
        }
    }

    pub fn draw(&mut self) {
        self.renderer.present();
    }

    pub fn clear(&mut self) { 
        self.buffer = [0; 64*32];
    }
}
