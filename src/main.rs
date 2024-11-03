use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, keyboard::Scancode};

use std::time::{Duration, Instant};

mod apple;
use apple::Apple;

mod coords;

mod snek;
use snek::{Direction, Snake};

const BLACK: Color = Color::RGB(0, 0, 0);

fn main() {
    let sdl_context = sdl2::init().expect("Shouldn't have failed");
    let video_subsystem = sdl_context.video().expect("huh?");

    let window = video_subsystem
        .window("Snek", 800, 600)
        .fullscreen_desktop()
        .build()
        .expect("how?");

    coords::init(window.size(), snek::BOX_SIZE);

    let mut canvas = window.into_canvas().build().expect("Shouldn't have failed");
    let mut event_pump = sdl_context.event_pump().expect("huh?");

    let mut time = Instant::now();
    let mut paused = true;
    let mut snake = Snake::new((0, 0), Direction::Right);

    let mut apple = [
        Apple::new(coords::width(), coords::height()),
        Apple::new(coords::width(), coords::height()),
        Apple::new(coords::width(), coords::height()),
        Apple::new(coords::width(), coords::height()),
        Apple::new(coords::width(), coords::height()),
    ];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => paused = !paused,
                _ => (),
            }
        }

        event_pump
            .keyboard_state()
            .pressed_scancodes()
            .for_each(|x| match x {
                Scancode::W | Scancode::Up => snake.change_direction(Direction::Up),
                Scancode::A | Scancode::Left => snake.change_direction(Direction::Left),
                Scancode::S | Scancode::Down => snake.change_direction(Direction::Down),
                Scancode::D | Scancode::Right => snake.change_direction(Direction::Right),
                _ => (),
            });

        update(&mut snake, &mut apple);

        if !paused {
            snake.move_snek(&mut time).expect("LMAO LOSER");
        }

        canvas.set_draw_color(BLACK);
        canvas.clear();
        draw_snek(&mut canvas, snake.get_boxes());
        draw_apple(&mut canvas, &apple);
        canvas.present();

        std::thread::sleep(Duration::from_nanos(1_000_000_000 / 60));
    }
}

fn update(snake: &mut Snake, apple: &mut [Apple; 5]) {
    let head_pos = snake.head();

    for i in apple {
        if i.0 == head_pos {
            *i = Apple::new(coords::width(), coords::height());
            snake.ate();
        }
    }
}

fn draw_apple(canvas: &mut Canvas<Window>, apple: &[Apple; 5]) {
    canvas.set_draw_color(Color::RGB(255, 128, 128));
    for i in apple {
        canvas.fill_rect(i.get_box()).expect("huh");
    }
}

fn draw_snek(canvas: &mut Canvas<Window>, boxes: Vec<sdl2::rect::Rect>) {
    boxes.into_iter().enumerate().for_each(|(i, r)| {
        if i == 0 {
            canvas.set_draw_color(Color::RGB(64, 156, 16));
            canvas.fill_rect(r).expect("huh");
            canvas.set_draw_color(Color::RGB(128, 255, 64));
        } else {
            canvas.fill_rect(r).expect("huh");
        }
    });
}
