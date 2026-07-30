use std::{f32, time::Duration};

use cgmath::{Matrix4, Rad, Vector2, Vector3, perspective, prelude::*};
use winit::dpi::PhysicalSize;

use crate::render::rasterize::rasterize_triangle;

mod rasterize;

pub fn render(pixel_buffer: &mut [u8], buffer_size: PhysicalSize<u32>, time: Duration) {
    // right handed coordinates
    let cube_triangles = [
        // +z face
        [
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(0.5, 0.5, 0.5),
        ],
        [
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
        ],
        // -z face
        [
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(-0.5, 0.5, -0.5),
        ],
        [
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
        ],
        // +x face
        [
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(0.5, 0.5, -0.5),
        ],
        [
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(0.5, -0.5, 0.5),
        ],
        // -x face
        [
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(-0.5, 0.5, 0.5),
        ],
        [
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(-0.5, -0.5, -0.5),
        ],
        // +y face
        [
            Vector3::new(-0.5, 0.5, 0.5),
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::new(0.5, 0.5, -0.5),
        ],
        [
            Vector3::new(0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, -0.5),
            Vector3::new(-0.5, 0.5, 0.5),
        ],
        // -y face
        [
            Vector3::new(-0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, -0.5),
            Vector3::new(0.5, -0.5, 0.5),
        ],
        [
            Vector3::new(0.5, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, 0.5),
            Vector3::new(-0.5, -0.5, -0.5),
        ],
    ];

    let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, -2.0))
        * Matrix4::from_angle_x(Rad(time.as_secs_f32().sin()))
        * Matrix4::from_angle_y(Rad(time.as_secs_f32()));
    let view_matrix = Matrix4::<f32>::one();
    let proj_matrix = perspective(
        Rad(f32::consts::PI / 2.0),
        buffer_size.width as f32 / buffer_size.height as f32,
        0.1,
        50.0,
    );

    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[0, 0, 0, 0xFF]);
    }

    for tri in cube_triangles {
        let wv0 = proj_matrix * view_matrix * model_matrix * tri[0].extend(1.0);
        let wv1 = proj_matrix * view_matrix * model_matrix * tri[1].extend(1.0);
        let wv2 = proj_matrix * view_matrix * model_matrix * tri[2].extend(1.0);

        let w0 = wv0.w;
        let w1 = wv1.w;
        let w2 = wv2.w;

        let inv_w0 = 1.0 / w0;
        let inv_w1 = 1.0 / w1;
        let inv_w2 = 1.0 / w2;

        let v0 = wv0 / w0;
        let v1 = wv1 / w1;
        let v2 = wv2 / w2;

        let color0 = Vector3::new(1.0, 0.0, 0.0) / w0;
        let color1 = Vector3::new(0.0, 1.0, 0.0) / w1;
        let color2 = Vector3::new(0.0, 0.0, 1.0) / w2;

        rasterize_triangle(
            v0.xy(),
            v1.xy(),
            v2.xy(),
            Vector2::new(buffer_size.width as i32, buffer_size.height as i32),
            |x: u32, y: u32, barycentric: Vector3<f32>| {
                let buf_idx = 4 * (y * buffer_size.width + x) as usize;
                let inv_w =
                    barycentric.x * inv_w0 + barycentric.y * inv_w1 + barycentric.z * inv_w2;

                /*let position =
                (v0 * barycentric.x + v1 * barycentric.y + v2 * barycentric.z) / inv_w;*/
                let color =
                    (color0 * barycentric.x + color1 * barycentric.y + color2 * barycentric.z)
                        / inv_w;
                pixel_buffer[buf_idx] = (color.x * 255.0).round() as u8;
                pixel_buffer[buf_idx + 1] = (color.y * 255.0).round() as u8;
                pixel_buffer[buf_idx + 2] = (color.z * 255.0).round() as u8;
                pixel_buffer[buf_idx + 3] = 0xFF;
            },
        );
    }
}
