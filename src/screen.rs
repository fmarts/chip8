use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Screen {
    pub renderer: Canvas<Window>,
    pub buffer: [u8; 64*32],
}

impl Screen { 
    pub fn new(ctx: &sdl2::Sdl) -> Screen {
        let video_subsystem = ctx.video().expect("Failed to initialize video subsystem");
        let window = video_subsystem.window("CHIP-8 Emulator", 640, 320)
            .position_centered()
            .build()
            .expect("Failed to create window");

        let mut canvas = window.into_canvas().build().expect("Failed to create canvas");
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Screen {
            renderer: canvas,
            buffer: [0; 64*32]
        }
    }

    pub fn draw(&mut self) {
        self.renderer.present();
    }

    pub fn clear(&mut self) { 
        self.buffer = [0; 64*32];
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
    }
}
