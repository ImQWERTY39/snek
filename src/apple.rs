use rand::Rng;

use crate::coords::Position;

pub const BOX_SIZE: u32 = 25;

pub struct Apple(pub Position);

impl Apple {
    pub fn new(width: i32, height: i32) -> Self {
        let mut random = rand::thread_rng();

        Self(Position {
            x: random.gen_range(0..width),
            y: random.gen_range(0..height),
        })
    }

    pub fn get_box(&self) -> sdl2::rect::Rect {
        let coords = self.0.to_window_coords();
        sdl2::rect::Rect::new(coords.0, coords.1, BOX_SIZE, BOX_SIZE)
    }
}
