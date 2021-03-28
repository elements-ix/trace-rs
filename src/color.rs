use crate::{util::clamp, vec3::Color};

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) -> Vec<u8> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    b *= scale;
    g *= scale;

    vec![
        (255.99 * clamp(r.sqrt(), 0.0, 0.999)) as u8,
        (255.99 * clamp(g.sqrt(), 0.0, 0.999)) as u8,
        (255.99 * clamp(b.sqrt(), 0.0, 0.999)) as u8,
        255,
    ]
}
