// src/bin/main.rs
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use indicatif::ProgressBar;
use png;
use rand::prelude::*;

use trace::hit::Hit;
use trace::ray::Ray;
use trace::sphere::Sphere;
use trace::vec3::{unit_vector, Color, Point3};
use trace::{camera::Camera, color::write_color};

fn ray_color(r: &Ray, world: &dyn Hit) -> Color {
    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        return 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut rng = thread_rng();

    // image
    let aspect_ratio = 16.0 / 9.0;
    let width = 600;
    let height = (width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // world
    let world: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // camera
    let cam = Camera::new();

    let bar = ProgressBar::new((width * height) as u64);

    let mut data = Vec::new();
    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(&mut data, pixel_color, samples_per_pixel);
            bar.inc(1);
        }
    }
    bar.finish();

    let path = Path::new(r"./images/chapter-7.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}
