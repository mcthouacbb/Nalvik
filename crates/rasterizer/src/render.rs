use std::{f32, time::Duration};

use cgmath::{Matrix4, Rad, Vector2, Vector3, Vector4, perspective, prelude::*};
use macros::VertexToFragment;
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera,
    render::pipeline::{Pipeline, VertexOutput},
    util::PERSPECTIVE_CORRECTION,
};

mod clip;
mod pipeline;
mod rasterize;
mod uniforms;

struct BasicVertexData {
    pos: Vector3<f32>,
    color: Vector3<f32>,
}

#[derive(Clone, Copy, VertexToFragment)]
struct BasicVertexOutput {
    color: Vector3<f32>,
}

fn vertex_shader(
    vertex_input: BasicVertexData,
    uniforms: &Matrix4<f32>,
) -> VertexOutput<BasicVertexOutput> {
    let out_pos = uniforms * vertex_input.pos.extend(1.0);
    VertexOutput {
        position: out_pos,
        data: BasicVertexOutput {
            color: vertex_input.color,
        },
    }
}

fn fragment_shader(fragment_input: BasicVertexOutput, _uniforms: ()) -> Vector4<f32> {
    if fragment_input.color.y <= 0.2
        && (fragment_input.color.x - fragment_input.color.z).abs() < 0.4
    {
        Vector3::from_value(1.0) - fragment_input.color
    } else {
        fragment_input.color
    }
    .extend(1.0)
}

pub fn render(
    pixel_buffer: &mut [u8],
    buffer_size: PhysicalSize<u32>,
    time: Duration,
    camera: &Camera,
) {
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

    let model_matrix1 =
        Matrix4::from_translation(Vector3::new(1.5, 0.0, -3.0 * time.as_secs_f32().sin()))
            * Matrix4::from_angle_x(Rad(time.as_secs_f32().sin()))
            * Matrix4::from_angle_y(Rad(time.as_secs_f32()));
    let model_matrix2 = Matrix4::from_translation(Vector3::new(-1.5, 0.0, -3.0))
        * Matrix4::from_angle_x(Rad(-(1.234 * time.as_secs_f32()).sin()))
        * Matrix4::from_angle_y(Rad(-0.8 * time.as_secs_f32()));
    let model_matrix3 =
        Matrix4::from_translation(Vector3::new(0.0, 0.0, -3.0 * time.as_secs_f32().sin()))
            * Matrix4::from_angle_x(Rad(-(1.234 * time.as_secs_f32()).sin()))
            * Matrix4::from_angle_y(Rad(-0.8 * time.as_secs_f32()));
    let view_matrix = camera.view_matrix();
    let proj_matrix = PERSPECTIVE_CORRECTION
        * perspective(
            Rad(f32::consts::PI / 3.0),
            buffer_size.width as f32 / buffer_size.height as f32,
            1.0,
            50.0,
        );

    let pipeline = Pipeline::new(vertex_shader, fragment_shader);

    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[0, 0, 0, 0xFF]);
    }

    for i in 0..3 {
        let model_matrix = if i == 0 {
            &model_matrix1
        } else if i == 1 {
            &model_matrix2
        } else {
            &model_matrix3
        };

        for tri in cube_triangles {
            let vertex_data0 = BasicVertexData {
                pos: tri[0],
                color: Vector3::new(1.0, 0.0, 0.0),
            };
            let vertex_data1 = BasicVertexData {
                pos: tri[1],
                color: Vector3::new(0.0, 1.0, 0.0),
            };
            let vertex_data2 = BasicVertexData {
                pos: tri[2],
                color: Vector3::new(0.0, 0.0, 1.0),
            };

            let mvp = proj_matrix * view_matrix * model_matrix;
            pipeline.run(
                &mvp,
                (),
                vertex_data0,
                vertex_data1,
                vertex_data2,
                Vector2::new(buffer_size.width as i32, buffer_size.height as i32),
                |x, y, color| {
                    let buf_idx = 4 * (y * buffer_size.width + x) as usize;
                    pixel_buffer[buf_idx] = (color.x * 255.0).round() as u8;
                    pixel_buffer[buf_idx + 1] = (color.y * 255.0).round() as u8;
                    pixel_buffer[buf_idx + 2] = (color.z * 255.0).round() as u8;
                    pixel_buffer[buf_idx + 3] = (color.w * 255.0).round() as u8;
                },
            );
        }
    }
}
