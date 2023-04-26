use std::f64::consts::PI;
use std::time;

use clgl;

// const CHARSET: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const CHARSET: &[u8] = b" `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const WIDTH: usize = 256usize;
const HEIGHT: usize = 128usize;
const g: f64 = 9.818f64;

#[derive(Debug)]
struct DoublePendulum {
    m1: f64,
    m2: f64,
    l1: f64,
    l2: f64,
    a1: f64,
    a2: f64,
    a1_vel: f64,
    a2_vel: f64,
    a1_acc: f64,
    a2_acc: f64,
    brightness: f64,
}

impl DoublePendulum {
    pub fn new(m1: f64,
               m2: f64,
               l1: f64,
               l2: f64,
               a1: f64,
               a2: f64,
               brightness: f64) -> Self {
        Self {
            m1,
            m2,
            l1,
            l2,
            a1,
            a2,
            a1_vel: 0.0f64,
            a2_vel: 0.0f64,
            a1_acc: 0.0f64,
            a2_acc: 0.0f64,
            brightness,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.a1_acc = (-g * (2.0f64 * self.m1 + self.m2) * self.a1.sin() - self.m2 * g * (self.a1 - 2.0f64 * self.a2).sin() - 2.0f64 * (self.a1 - self.a2).sin() * self.m2 * (self.a2_vel.powi(2i32) * self.l2 + self.a1_vel.powi(2i32) * self.l1 * (self.a1 - self.a2).cos())) / (self.l1 * (2.0f64 * self.m1 + self.m2 - self.m2 * (2.0f64 * self.a1 - 2.0f64 * self.a2).cos()));
        self.a2_acc = (2.0f64 * (self.a1 - self.a2).sin() * (self.a1_vel.powi(2i32) * self.l1 * (self.m1 + self.m2) + g * (self.m1 + self.m2) * self.a1.cos() + self.a2_vel.powi(2i32) * self.l2 * self.m2 * (self.a1 - self.a2).cos())) / (self.l2 * (2.0f64 * self.m1 + self.m2 - self.m2 * (2.0f64 * self.a1 - 2.0f64 * self.a2).cos()));

        self.a1 += self.a1_acc * 0.5f64 * dt.powi(2i32) + self.a1_vel * dt;
        self.a1_vel += self.a1_acc * dt;

        self.a2 += self.a2_acc * 0.5f64 * dt.powi(2i32) + self.a2_vel * dt;
        self.a2_vel += self.a2_acc * dt;
    }

    pub fn draw(&self, canvas: &mut clgl::canvas::Canvas) {
        let cx: f64 = WIDTH as f64 * 0.5f64;
        let cy: f64 = HEIGHT as f64 * 0.5f64;

        let p1x: f64 = cx + (self.a1 + PI * 0.5f64).cos() * self.l1;
        let p1y: f64 = cy + (self.a1 + PI * 0.5f64).sin() * self.l1;

        let p2x: f64 = p1x + (self.a2 + PI * 0.5f64).cos() * self.l2;
        let p2y: f64 = p1y + (self.a2 + PI * 0.5f64).sin() * self.l2;

        clgl::draw::circle(canvas, cx, cx, 1.5f64, self.brightness);
        clgl::draw::line_segment(canvas, cx, cy, p1x, p1y, self.brightness);
        clgl::draw::circle(canvas, p1x, p1y, 1.5f64, self.brightness);
        clgl::draw::line_segment(canvas, p1x, p1y, p2x, p2y, self.brightness);
        clgl::draw::circle(canvas, p2x, p2y, 1.5f64, self.brightness)
    }
}

fn main() {
    let mut canvas: clgl::canvas::Canvas = clgl::canvas::Canvas::new(WIDTH, HEIGHT, CHARSET);
    canvas.equalize_row_column_spacing = true;

    let mut double_pendulums: Vec<DoublePendulum> = Vec::new();

    let l1: f64 = HEIGHT as f64 * 0.25;
    let l2: f64 = HEIGHT as f64 * 0.25;

    double_pendulums.push(DoublePendulum::new(1.0f64, 1.0f64, l1, l2, PI * 0.5f64, 0.0f64, 1.0f64));
    double_pendulums.push(DoublePendulum::new(1.0f64, 1.0f64, l1, l2, PI * 0.501f64, 0.0f64, 0.75f64));
    double_pendulums.push(DoublePendulum::new(1.0f64, 1.0f64, l1, l2, PI * 0.499f64, 0.0f64, 0.5f64));
    double_pendulums.push(DoublePendulum::new(1.0f64, 1.0f64, l1, l2, PI, PI * 0.5f64, 0.25f64));

    let mut last_frame_time: time::Instant = time::Instant::now();
    loop {
        let frame_time_start: time::Instant = time::Instant::now();
        let dt: f64 = frame_time_start
            .duration_since(last_frame_time)
            .as_secs_f64();
        last_frame_time = frame_time_start;

        clgl::draw::fill(&mut canvas, 0.0f64);
        // canvas.map_pixels(|x: f64| x * 0.99);

        for double_pendulum in double_pendulums.iter_mut().rev() {
            double_pendulum.draw(&mut canvas);
            double_pendulum.update(dt);
        };


        clgl::tools::reset_cursor_position();
        canvas.render();
        print!("fps: {}", 1.0f64 / dt);
    };
}