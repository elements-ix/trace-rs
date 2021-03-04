use crate::vec3::Color;

pub fn write_color(data: &mut Vec<u8>, pixel_color: Color) {
    data.extend_from_slice(&[
        (255.99 * pixel_color.x()) as u8,
        (255.99 * pixel_color.y()) as u8,
        (255.99 * pixel_color.z()) as u8,
        255,
    ]);
}
