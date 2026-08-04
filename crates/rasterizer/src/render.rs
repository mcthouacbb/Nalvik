use std::{f32, time::Duration};

use cgmath::{Matrix4, Rad, Vector2, Vector3, Vector4, perspective};
use macros::VertexToFragment;
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera,
    render::{
        image::{
            Image,
            format::{DepthF32, RgbaU8},
            view::ImageViewMut,
        },
        pipeline::{
            Pipeline, VertexOutput,
            depth_state::{DepthState, DepthTest},
        },
    },
    util::PERSPECTIVE_CORRECTION,
};

mod clip;
mod image;
mod pipeline;
mod rasterize;
mod uniforms;

#[derive(Clone, Copy)]
struct BasicVertexData {
    pos: Vector3<f32>,
    color: Vector3<f32>,
}

impl BasicVertexData {
    pub fn new(pos: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { pos, color }
    }
}

#[derive(Clone, Copy, VertexToFragment)]
struct BasicVertexOutput {
    color: Vector3<f32>,
}

fn vertex_shader(
    vertex_input: &BasicVertexData,
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

fn fragment_shader(fragment_input: &BasicVertexOutput, _uniforms: ()) -> Vector4<f32> {
    fragment_input.color.extend(1.0)
}

fn cube_mesh() -> [[BasicVertexData; 3]; 12] {
    // right handed coordinates
    [
        // +z face
        [
            BasicVertexData::new(Vector3::new(-0.5, -0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        // -z face
        [
            BasicVertexData::new(Vector3::new(0.5, -0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(-0.5, 0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        // +x face
        [
            BasicVertexData::new(Vector3::new(0.5, -0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(0.5, 0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        // -x face
        [
            BasicVertexData::new(Vector3::new(-0.5, -0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(-0.5, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        // +y face
        [
            BasicVertexData::new(Vector3::new(-0.5, 0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, 0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(0.5, 0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, 0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        // -y face
        [
            BasicVertexData::new(Vector3::new(-0.5, -0.5, -0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, -0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(0.5, -0.5, 0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
        [
            BasicVertexData::new(Vector3::new(0.5, -0.5, 0.5), Vector3::new(1.0, 0.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, 0.5), Vector3::new(0.0, 1.0, 0.0)),
            BasicVertexData::new(Vector3::new(-0.5, -0.5, -0.5), Vector3::new(0.0, 0.0, 1.0)),
        ],
    ]
}

fn overlapping_tri_mesh() -> [[BasicVertexData; 3]; 2] {
    [
        [
            BasicVertexData::new(
                Vector3::new(-0.5, 0.5, 0.2),
                Vector3::new(0.054, 0.242, 0.913),
            ),
            BasicVertexData::new(
                Vector3::new(-0.5, -0.5, 0.2),
                Vector3::new(0.054, 0.242, 0.913),
            ),
            BasicVertexData::new(
                Vector3::new(1.4, 0.0, -0.2),
                Vector3::new(0.054, 0.242, 0.913),
            ),
        ],
        [
            BasicVertexData::new(
                Vector3::new(0.5, -0.5, 0.2),
                Vector3::new(0.209, 0.791, 0.036),
            ),
            BasicVertexData::new(
                Vector3::new(0.5, 0.5, 0.2),
                Vector3::new(0.209, 0.791, 0.036),
            ),
            BasicVertexData::new(
                Vector3::new(-1.4, 0.0, -0.2),
                Vector3::new(0.209, 0.791, 0.036),
            ),
        ],
    ]
}

pub struct Renderer {
    depth_buffer: Image<DepthF32>,
    size: Vector2<i32>,
    cube: [[BasicVertexData; 3]; 12],
    overlapping_tris: [[BasicVertexData; 3]; 2],
}

impl Renderer {
    pub fn new(viewport_size: PhysicalSize<u32>) -> Self {
        Self {
            depth_buffer: Image::new(
                DepthF32::new(1.0),
                viewport_size.width,
                viewport_size.height,
            ),
            size: Vector2::new(viewport_size.width as i32, viewport_size.height as i32),
            cube: cube_mesh(),
            overlapping_tris: overlapping_tri_mesh(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = Vector2::new(new_size.width as i32, new_size.height as i32);
        self.depth_buffer = Image::new(DepthF32::new(1.0), new_size.width, new_size.height);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size.x as f32 / self.size.y as f32
    }
}

pub fn render(renderer: &mut Renderer, pixel_buffer: &mut [u8], time: Duration, camera: &Camera) {
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
            renderer.aspect_ratio(),
            0.1,
            50.0,
        );

    let pipeline = Pipeline::new(vertex_shader, fragment_shader);

    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[108, 182, 204, 0xFF]);
    }

    let mut framebuffer = ImageViewMut::new(
        bytemuck::cast_slice_mut::<u8, RgbaU8>(pixel_buffer),
        renderer.size.x as u32,
        renderer.size.y as u32,
    );

    let mut depth_buffer = Image::new(
        DepthF32::from(1.0),
        renderer.size.x as u32,
        renderer.size.y as u32,
    );
    let mut depth_state = DepthState::CompareAndWrite(depth_buffer.view_mut(), DepthTest::Less);

    for i in 0..3 {
        let model_matrix = if i == 0 {
            &model_matrix1
        } else if i == 1 {
            &model_matrix2
        } else {
            &model_matrix3
        };

        for tri in &renderer.cube {
            let mvp = proj_matrix * view_matrix * model_matrix;
            pipeline.run(
                &mvp,
                (),
                &tri[0],
                &tri[1],
                &tri[2],
                renderer.size,
                &mut framebuffer,
                &mut depth_state,
            );
        }
    }

    let model_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, -5.0));
    for tri in renderer.overlapping_tris {
        let mvp = proj_matrix * view_matrix * model_matrix;
        pipeline.run(
            &mvp,
            (),
            &tri[0],
            &tri[1],
            &tri[2],
            renderer.size,
            &mut framebuffer,
            &mut depth_state,
        );
    }
}
