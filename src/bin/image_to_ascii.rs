use std::path::Path;

use clgl;
use image::{self, GenericImageView, Pixel};

const CHARSET: &[u8] = b" `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let mut img: image::DynamicImage = image::open(Path::new("./resources/ln.png")).unwrap();
    img = img.resize(WIDTH as u32, HEIGHT as u32, image::imageops::FilterType::Nearest).grayscale();

    let mut canvas: clgl::canvas::Canvas = clgl::canvas::Canvas::new(img.width() as usize, img.height() as usize, CHARSET);
    canvas.equalize_row_column_spacing = true;

    img.pixels().for_each(|(x, y, rgba)| {
        canvas.set_pixel(x as f64, y as f64, rgba.to_luma()[0] as f64 / 255.0f64);
    });

    canvas.render();
}