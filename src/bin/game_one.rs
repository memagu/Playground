use clgl;
use std::{thread, time};

const CHARSET: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const WIDTH: usize = 256;
const HEIGHT: usize = 64;
const MAX_FRAME_RATE: f64 = 1024.0f64;

fn sleep(seconds: f64) {
    let duration: time::Duration = time::Duration::from_secs_f64(seconds);
    thread::sleep(duration);
}

struct Rect {
    pub pos_x: f64,
    pub pos_y: f64,
    pub width: f64,
    pub height: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub acc_x: f64,
    pub acc_y: f64,
    pub brightness: f64,
}

impl Rect {
    pub fn new(
        pos_x: f64,
        pos_y: f64,
        width: f64,
        height: f64,
        vel_x: f64,
        vel_y: f64,
        acc_x: f64,
        acc_y: f64,
        brightness: f64,
    ) -> Self {
        Self {
            pos_x,
            pos_y,
            width,
            height,
            vel_x,
            vel_y,
            acc_x,
            acc_y,
            brightness,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.vel_x += self.acc_x * dt;
        self.vel_y += self.acc_y * dt;
        self.pos_x += self.vel_x * dt;
        self.pos_y += self.vel_y * dt;
    }

    pub fn check_and_resolve_edge_bounce(&mut self, canvas_width: f64, canvas_height: f64) {
        if self.pos_x < 0.0f64 {
            self.vel_x = self.vel_x.abs()
        } else if self.pos_x + self.width >= canvas_width {
            self.vel_x = -self.vel_x.abs();
        }

        if self.pos_y < 0.0f64 {
            self.vel_y = self.vel_y.abs();
            self.acc_y = self.acc_y.abs();
        } else if self.pos_y + self.height >= canvas_height {
            self.vel_y = -self.vel_y.abs();
            self.acc_y = -self.acc_y.abs();
        }
    }

    pub fn draw(&self, canvas: &mut clgl::canvas::Canvas) {
        clgl::draw::rectangle(
            canvas,
            self.pos_x,
            self.pos_y,
            self.width,
            self.height,
            self.brightness,
        )
    }
}

fn main() {
    let mut run: bool = true;

    let mut canvas: clgl::canvas::Canvas = clgl::canvas::Canvas::new(WIDTH, HEIGHT, CHARSET);

    let mut rects: Vec<Rect> = Vec::new();

    rects.push(Rect::new(
        0.0f64,
        (HEIGHT as f64 - 3.0f64) / 2.0f64,
        5.0f64,
        3.0f64,
        10.064,
        0.0f64,
        0.0f64,
        0.0f64,
        1.0f64,
    ));

    rects.push(Rect::new(
        16.0f64, 8.0f64, 6.0f64, 4.0f64, 6.064f64, 7.3f64, 0.0f64, 0.0f64, 0.5f64,
    ));

    rects.push(Rect::new(
        24.0f64, 24.0f64, 2.0f64, 2.0f64, 10.064, -5.0f64, 0.0f64, 0.0f64, 0.25f64,
    ));

    rects.push(Rect::new(
        24.0f64, 24.0f64, 3.0f64, 3.0f64, 100.064, -53.0f64, 0.0f64, 0.0f64, 0.125f64,
    ));

    rects.push(Rect::new(
        24.0f64, 24.0f64, 3.0f64, 3.0f64, 100.064, -53.0f64, 5.0f64, 4.0f64, 0.125f64,
    ));

    let mut last_frame_time: time::Instant = time::Instant::now();
    while run {
        let frame_time_start: time::Instant = time::Instant::now();
        let dt: f64 = frame_time_start
            .duration_since(last_frame_time)
            .as_secs_f64();
        last_frame_time = frame_time_start;

        // clgl::draw::fill(&mut canvas, 0.0f64);
        canvas.map_pixels(|x| x - 0.001);

        for i in 0..rects.len() {
            rects[i].check_and_resolve_edge_bounce(WIDTH as f64, HEIGHT as f64);
            rects[i].update(dt);
            rects[i].draw(&mut canvas);
            for j in 0..rects.len() {
                if i == j {
                    continue;
                }
                // clgl::draw::line_segment(
                //     &mut canvas,
                //     rects[i].pos_x,
                //     rects[i].pos_y,
                //     rects[j].pos_x,
                //     rects[j].pos_y,
                //     (rects[i].brightness + rects[j].brightness) / 2.0f64,
                // )
            }
        }

        // clgl::draw::line_segment(&mut canvas, 0.0f64, 0.0f64, WIDTH as f64 - 1.0f64, 0.0f64, 0.1f64);
        // clgl::draw::line_segment(&mut canvas, WIDTH as f64 - 1.0f64, 0.0f64, WIDTH as f64 - 1.0f64, WIDTH as f64 - 1.0f64, 0.1f64);
        // clgl::draw::line_segment(&mut canvas, 0.0f64, WIDTH as f64 - 1.0f64, WIDTH as f64 - 1.0f64, WIDTH as f64 - 1.0f64, 0.1f64);
        // clgl::draw::line_segment(&mut canvas, 0.0f64, 0.0f64, 0.0f64, WIDTH as f64 - 1.0f64, 0.1f64);

        // Draw
        clgl::tools::reset_cursor_position();
        canvas.render(); // Render Canvas
        print!("{}", 1.0f64 / dt);

        if dt < 1.0f64 / MAX_FRAME_RATE {
            sleep(1.0f64 / MAX_FRAME_RATE - dt);
        }
    }
}
