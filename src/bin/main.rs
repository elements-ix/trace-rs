// src/bin/main.rs
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use indicatif::ProgressBar;
use png;

use trace::color::write_color;
use trace::vec3::Color;

fn main() {
    let width = 600;
    let height = 400;

    let bar = ProgressBar::new((width * height) as u64);

    let mut data = Vec::new();
    for j in (0..height).rev() {
        for i in 0..width {
            let pixel_color = Color::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.25,
            );
            write_color(&mut data, pixel_color);
            bar.inc(1);
        }
    }
    bar.finish();

    let path = Path::new(r"./images/chapter-3.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}
