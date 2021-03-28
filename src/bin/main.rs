// src/bin/main.rs
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png;
use rand::prelude::*;
use rayon::prelude::*;

use tracers::sphere::Sphere;
use tracers::vec3::{random_unit_vector, unit_vector, Color, Point3, Vec3};
use tracers::{camera::Camera, color::write_color};
use tracers::{
    hit::Hit,
    material::{Lambertian, Metal},
};
use tracers::{material::Dielectric, ray::Ray};

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

fn random_scene(rng: &mut ThreadRng) -> Vec<Box<dyn Hit>> {
    let mut world: Vec<Box<dyn Hit>> = Vec::new();
    let ground_material = Box::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.push(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Box::new(Lambertian { albedo });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Box::new(Metal { albedo, fuzz });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else {
                    // glass
                    let sphere_material = Box::new(Dielectric { ir: 1.5 });
                    world.push(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                }
            }
        }
    }

    let large_glass_material = Box::new(Dielectric { ir: 1.5 });
    world.push(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: large_glass_material,
    }));

    let large_diffuse_material = Box::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.push(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: large_diffuse_material,
    }));

    let large_metal_material = Box::new(Metal {
        albedo: Point3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: large_metal_material,
    }));

    world
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
    let world = random_scene(&mut rng);

    // camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // let bar = ProgressBar::new((width * height) as u64);

    let mut data = Vec::new();
    let image = (0..height as usize)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..width)
                .into_par_iter()
                .flat_map(|i| {
                    let mut rng = thread_rng();

                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen::<f64>()) / (width - 1) as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / (height as usize - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &world, max_depth);
                    }
                    write_color(pixel_color, samples_per_pixel)
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for col in image.chunks(4) {
        let col_vec = col.to_vec();
        data.extend_from_slice(col_vec.as_slice());
    }

    let path = Path::new(r"./images/chapter-13.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(data.as_slice()).unwrap();
}
