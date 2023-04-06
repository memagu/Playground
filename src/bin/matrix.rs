use clgl;
use rand::{self, Rng};
use std::{thread, time};

const CHARSET: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const WIDTH: usize = 256;
const HEIGHT: usize = 64;
const MAX_FRAME_RATE: f64 = 1024.0f64;

fn sleep(seconds: f64) {
    let duration: time::Duration = time::Duration::from_secs_f64(seconds);
    thread::sleep(duration);
}

struct Point {
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub brightness: f64,
}

impl Point {
    pub fn new(pos_x: f64, pos_y: f64, vel_x: f64, vel_y: f64, brightness: f64) -> Self {
        Self {
            pos_x,
            pos_y,
            vel_x,
            vel_y,
            brightness,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.pos_x += self.vel_x * dt;
        self.pos_y += self.vel_y * dt;
    }

    pub fn draw(&self, canvas: &mut clgl::canvas::Canvas) {
        clgl::draw::rectangle(
            canvas,
            self.pos_x,
            self.pos_y,
            1.0f64,
            1.0f64,
            self.brightness,
        )
    }
}

fn create_random_falling_point(max_x: f64, max_y: f64, y_vel: f64) -> Point {
    let x: f64 = rand::thread_rng().gen_range(0.0f64..max_x);
    let y: f64 = rand::thread_rng().gen_range(0.0f64..max_y);
    Point::new(x, y, 0.0f64, y_vel, 1.0f64)
}

fn main() {
    let mut run: bool = true;

    let mut canvas: clgl::canvas::Canvas = clgl::canvas::Canvas::new(WIDTH, HEIGHT, CHARSET);

    let mut points: Vec<Point> = Vec::new();

    for _ in 0..WIDTH {
        points.push(create_random_falling_point(
            WIDTH as f64,
            HEIGHT as f64,
            10.0f64,
        ))
    }

    let mut last_frame_time: time::Instant = time::Instant::now();
    while run {
        let frame_time_start: time::Instant = time::Instant::now();
        let dt: f64 = frame_time_start
            .duration_since(last_frame_time)
            .as_secs_f64();
        last_frame_time = frame_time_start;

        // clgl::draw::fill(&mut canvas, 0.0f64);
        canvas.map_pixels(|x| x * 0.9);

        for i in 0..WIDTH {
            if points[i].pos_y >= HEIGHT as f64 {
                points[i] = create_random_falling_point(WIDTH as f64, HEIGHT as f64, 10.0f64);
            }
            let point: &mut Point = &mut points[i];
            point.draw(&mut canvas);
            point.update(dt);
        }

        // Draw
        clgl::tools::reset_cursor_position();
        canvas.render(); // Render Canvas
        print!("{}", 1.0f64 / dt);

        if dt < 1.0f64 / MAX_FRAME_RATE {
            sleep(1.0f64 / MAX_FRAME_RATE - dt);
        }
    }
}
