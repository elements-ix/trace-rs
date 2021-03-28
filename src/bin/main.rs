// src/bin/main.rs
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use indicatif::ProgressBar;
use png;
use rand::prelude::*;

use trace::sphere::Sphere;
use trace::vec3::{random_unit_vector, unit_vector, Color, Point3, Vec3};
use trace::{camera::Camera, color::write_color};
use trace::{
    hit::Hit,
    material::{Lambertian, Metal},
};
use trace::{material::Dielectric, ray::Ray};

fn ray_color(r: &Ray, world: &dyn Hit, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some(scattered) = hit.mat.scatter(&r, &hit) {
            return scattered.attenuation * ray_color(&scattered.scattered, world, depth - 1);
        }
        let target = hit.p + hit.normal + random_unit_vector();
        return 0.5 * ray_color(&Ray::new(hit.p, target - hit.p), world, depth - 1);
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
    let max_depth = 50;

    // world
    let material_ground = Box::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Box::new(Dielectric { ir: 1.5 });
    let material_right = Box::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let world: Vec<Box<dyn Hit>> = vec![
        Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.0),
            0.5,
            material_center,
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )),
    ];

    // camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    let bar = ProgressBar::new((width * height) as u64);

    let mut data = Vec::new();
    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut data, pixel_color, samples_per_pixel);
            bar.inc(1);
        }
    }
    bar.finish();

    let path = Path::new(r"./images/chapter-11-3.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}
