use cgmath::{Vector2, Vector3};
use winit::dpi::PhysicalSize;

use crate::render::rasterize::rasterize_triangle;

mod rasterize;

pub fn render(pixel_buffer: &mut [u8], buffer_size: PhysicalSize<u32>) {
    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[0, 0, 0, 0xFF]);
    }

    let pixel_fn = |x: u32, y: u32, barycentric: Vector3<f32>| {
        let base = 4 * (y * buffer_size.width + x) as usize;
        pixel_buffer[base] = (barycentric.x * 255.0).round() as u8;
        pixel_buffer[base + 1] = (barycentric.y * 255.0).round() as u8;
        pixel_buffer[base + 2] = (barycentric.z * 255.0).round() as u8;
        pixel_buffer[base + 3] = 0xFF;
    };

    rasterize_triangle(
        Vector2::new(0.0, -0.5),
        Vector2::new(0.5, 0.0),
        Vector2::new(-0.5, 0.5),
        Vector2::new(buffer_size.width as i32, buffer_size.height as i32),
        pixel_fn,
    );
}
