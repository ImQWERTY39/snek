static mut WINDOW_WIDTH: u32 = 0;
static mut WINDOW_HEIGHT: u32 = 0;
static mut SCALE: i32 = 0;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn out_of_bounds(&self) -> bool {
        self.x < 0 || self.x > width() || self.y < 0 || self.y > height()
    }

    pub fn to_window_coords(&self) -> (i32, i32) {
        unsafe { (self.x * SCALE, self.y * SCALE) }
    }
}

pub fn init(size: (u32, u32), box_size: u32) {
    unsafe {
        (WINDOW_WIDTH, WINDOW_HEIGHT) = size;
        SCALE = box_size as i32;
    }
}

pub fn width() -> i32 {
    unsafe { WINDOW_WIDTH as i32 / SCALE }
}

pub fn height() -> i32 {
    unsafe { WINDOW_HEIGHT as i32 / SCALE }
}
