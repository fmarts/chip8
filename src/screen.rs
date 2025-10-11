use minifb::{Window, WindowOptions, Scale, Key};

pub struct Screen {
    pub window: Window,
    pub buffer: [u8; 64*32],
    pub pixel_buffer: Vec<u32>,
}

impl Screen { 
    pub fn new() -> Screen {
        let window = Window::new(
            "CHIP-8 Emulator",
            640,
            320,
            WindowOptions {
                scale: Scale::X1,
                ..WindowOptions::default()
            },
        ).expect("Failed to create window");

        // Initialize pixel buffer (RGB888 format) - scaled to 640x320
        let pixel_buffer = vec![0u32; 640 * 320];
        
        Screen {
            window,
            buffer: [0; 64*32],
            pixel_buffer,
        }
    }

    pub fn draw(&mut self) {
        // Convert CHIP-8 buffer to RGB888 pixel buffer with 10x scaling
        for y in 0..32 {
            for x in 0..64 {
                let color = if self.buffer[x + y * 64] == 1 {
                    0xFFFFFF // White
                } else {
                    0x000000 // Black
                };
                
                // Scale each pixel to 10x10
                for sy in 0..10 {
                    for sx in 0..10 {
                        let scaled_x = x * 10 + sx;
                        let scaled_y = y * 10 + sy;
                        self.pixel_buffer[scaled_x + scaled_y * 640] = color;
                    }
                }
            }
        }

        self.window.update_with_buffer(&self.pixel_buffer, 640, 320)
            .expect("Failed to update window buffer");
    }

    pub fn clear(&mut self) { 
        self.buffer = [0; 64*32];
        self.pixel_buffer = vec![0u32; 640 * 320];
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn get_keys(&self) -> Vec<Key> {
        self.window.get_keys()
    }
}