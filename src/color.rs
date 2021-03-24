use crate::{util::clamp, vec3::Color};

pub fn write_color(data: &mut Vec<u8>, pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    data.extend_from_slice(&[
        (255.99 * clamp(r, 0.0, 0.999)) as u8,
        (255.99 * clamp(g, 0.0, 0.999)) as u8,
        (255.99 * clamp(b, 0.0, 0.999)) as u8,
        255,
    ]);
}
