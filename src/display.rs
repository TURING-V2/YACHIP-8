use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, PIXEL_SIZE};

pub struct Display {
    display: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    draw_flag: bool,
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
            draw_flag: false,
        }
    }

    pub fn clear(&mut self) {
        self.display = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn set_draw_flag(&mut self, flag: bool) {
        self.draw_flag = flag;
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8], canvas: &mut Canvas<Window>) -> bool {
        let mut collision = false;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (i, row) in sprite.iter().enumerate() {
            for j in 0..8 {
                if (row >> (7 - j)) & 0x1 == 1 {
                    let index = (y + i) * SCREEN_WIDTH + (x + j);
                    if self.display[index] == 1 {
                        collision = true;
                    }
                    self.display[index] ^= 1;
                    let x_pos = (x + j) as i32;
                    let y_pos = (y + i) as i32;
                    let pixel_rect = Rect::new(x_pos * PIXEL_SIZE as i32, y_pos * PIXEL_SIZE as i32, PIXEL_SIZE, PIXEL_SIZE);
                    canvas.fill_rect(pixel_rect).unwrap();
                }
            }
        }
        collision
    }

    pub fn create_window_and_draw_to_screen<'a>(&mut self, sdl2_context: &'a sdl2::Sdl) -> Canvas<Window> {
        let video_subsystem = sdl2_context.video().unwrap();
        let window = video_subsystem
            .window("YACHIP8", SCREEN_WIDTH as u32 * PIXEL_SIZE as u32, SCREEN_HEIGHT as u32 * PIXEL_SIZE as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        self.draw(0, 0, &[0xFF, 0x80, 0x80, 0x80, 0xFF], &mut canvas);
        canvas.present();
        canvas
    }
}
