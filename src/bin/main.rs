// src/bin/main.rs
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png;

fn main() {
    let width = 600;
    let height = 400;

    let mut data = Vec::new();
    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.25;
            let ir = (255.99 * r as f64) as u8;
            let ig = (255.99 * g as f64) as u8;
            let ib = (255.99 * b as f64) as u8;
            data.extend_from_slice(&[ir, ig, ib, 255]);
        }
    }

    let path = Path::new(r"./images/chapter-2.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}