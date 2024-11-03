use std::{collections::VecDeque, time::Instant};

use crate::coords::Position;

pub const BOX_SIZE: u32 = 25;

pub struct Snake {
    body: VecDeque<Position>,
    direction: Direction,
    ate: bool,
}

impl Snake {
    pub fn new(pos: (i32, i32), direction: Direction) -> Self {
        let mut body = VecDeque::new();
        body.push_front(Position { x: pos.0, y: pos.1 });

        Self {
            body,
            direction,
            ate: false,
        }
    }

    pub fn move_snek(&mut self, time: &mut Instant) -> Result<(), ()> {
        if time.elapsed().as_millis() < 200 {
            return Ok(());
        }

        *time = Instant::now();

        let mut head_pos = self
            .body
            .front()
            .expect("there has to be atlease one")
            .clone();

        match self.direction {
            Direction::Up => head_pos.y -= 1,
            Direction::Down => head_pos.y += 1,
            Direction::Left => head_pos.x -= 1,
            Direction::Right => head_pos.x += 1,
        }

        if head_pos.out_of_bounds() {
            return Err(());
        }

        if self.ate {
            self.ate = false;
        } else {
            self.body.pop_back();
        }

        self.body.push_front(head_pos);

        Ok(())
    }

    pub fn get_boxes(&self) -> Vec<sdl2::rect::Rect> {
        let mut rects = Vec::with_capacity(self.body.len());

        self.body.iter().for_each(|i| {
            let coords = i.to_window_coords();
            rects.push(sdl2::rect::Rect::new(
                coords.0, coords.1, BOX_SIZE, BOX_SIZE,
            ))
        });

        rects
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if self.direction.opposite() == direction {
            return;
        }

        self.direction = direction;
    }

    pub fn head(&self) -> Position {
        self.body.front().expect("should be there").clone()
    }

    pub fn ate(&mut self) {
        self.ate = true;
    }
}

#[derive(PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
