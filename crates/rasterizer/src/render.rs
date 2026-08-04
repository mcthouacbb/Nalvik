use std::{f32, time::Duration};

use cgmath::{
    InnerSpace, Matrix, Matrix3, Matrix4, Rad, SquareMatrix, Vector2, Vector3, Vector4,
    perspective, vec2, vec3,
};
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
    terrain::{Terrain, generate_terrain},
    util::PERSPECTIVE_CORRECTION,
};

mod clip;
mod image;
mod pipeline;
mod rasterize;
mod uniforms;

#[derive(Clone, Copy)]
pub struct BasicVertexData {
    pos: Vector3<f32>,
    color: Vector3<f32>,
    normal: Vector3<f32>,
}

impl BasicVertexData {
    pub fn new(pos: Vector3<f32>, color: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, color, normal }
    }
}

#[derive(Clone, Copy, VertexToFragment)]
struct BasicVertexOutput {
    color: Vector3<f32>,
    normal: Vector3<f32>,
}

struct BasicUniforms {
    mvp_matrix: Matrix4<f32>,
    normal_matrix: Matrix3<f32>,
}

impl BasicUniforms {
    fn new(
        model_matrix: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        proj_matrix: &Matrix4<f32>,
    ) -> Self {
        Self {
            mvp_matrix: proj_matrix * view_matrix * model_matrix,
            normal_matrix: Matrix3::from_cols(
                model_matrix.x.xyz(),
                model_matrix.y.xyz(),
                model_matrix.z.xyz(),
            )
            .invert()
            .unwrap()
            .transpose(),
        }
    }
}

fn vertex_shader(
    vertex_input: &BasicVertexData,
    uniforms: &BasicUniforms,
) -> VertexOutput<BasicVertexOutput> {
    let out_pos = uniforms.mvp_matrix * vertex_input.pos.extend(1.0);
    let out_normal = (uniforms.normal_matrix * vertex_input.normal).normalize();
    VertexOutput {
        position: out_pos,
        data: BasicVertexOutput {
            color: vertex_input.color,
            normal: out_normal,
        },
    }
}

fn fragment_shader(fragment_input: &BasicVertexOutput, _uniforms: ()) -> Vector4<f32> {
    // (-0.4, -1, -0.5).normalized()
    const LIGHT_DIR: Vector3<f32> = Vector3::new(-0.336860768, -0.84215192, -0.42107596);
    let brightness = 0.5 * (fragment_input.normal.normalize().dot(-LIGHT_DIR) + 1.0);
    (fragment_input.color * brightness).extend(1.0)
}

fn cube_mesh() -> [[BasicVertexData; 3]; 12] {
    // right handed coordinates
    [
        // +z face
        [
            BasicVertexData::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, 1.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(0.5, 0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, 1.0),
            ),
        ],
        // -z face
        [
            BasicVertexData::new(
                vec3(0.5, -0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 0.0, -1.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, -1.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, -1.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 0.0, -1.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 0.0, -1.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, -1.0),
            ),
        ],
        // +x face
        [
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(1.0, 0.0, 0.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(0.5, 0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(1.0, 0.0, 0.0),
            ),
        ],
        // -x face
        [
            BasicVertexData::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(-1.0, 0.0, 0.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(-1.0, 0.0, 0.0),
            ),
        ],
        // +y face
        [
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 1.0, 0.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(0.5, 0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 1.0, 0.0),
            ),
        ],
        // -y face
        [
            BasicVertexData::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, -0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, -1.0, 0.0),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.5),
                vec3(1.0, 0.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, 0.5),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, -1.0, 0.0),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, -0.5),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, -1.0, 0.0),
            ),
        ],
    ]
}

fn overlapping_tri_mesh() -> [[BasicVertexData; 3]; 2] {
    [
        [
            BasicVertexData::new(
                vec3(-0.5, 0.5, 0.2),
                vec3(0.054, 0.242, 0.913),
                vec3(0.206010481, 0.0, 0.978549785),
            ),
            BasicVertexData::new(
                vec3(-0.5, -0.5, 0.2),
                vec3(0.054, 0.242, 0.913),
                vec3(0.206010481, 0.0, 0.978549785),
            ),
            BasicVertexData::new(
                vec3(1.4, 0.0, -0.2),
                vec3(0.054, 0.242, 0.913),
                vec3(0.206010481, 0.0, 0.978549785),
            ),
        ],
        [
            BasicVertexData::new(
                vec3(0.5, -0.5, 0.2),
                vec3(0.209, 0.791, 0.036),
                vec3(-0.206010481, 0.0, 0.978549785),
            ),
            BasicVertexData::new(
                vec3(0.5, 0.5, 0.2),
                vec3(0.209, 0.791, 0.036),
                vec3(-0.206010481, 0.0, 0.978549785),
            ),
            BasicVertexData::new(
                vec3(-1.4, 0.0, -0.2),
                vec3(0.209, 0.791, 0.036),
                vec3(-0.206010481, 0.0, 0.978549785),
            ),
        ],
    ]
}

pub struct Renderer {
    depth_buffer: Image<DepthF32>,
    size: Vector2<i32>,
    cube: [[BasicVertexData; 3]; 12],
    terrain: Terrain,
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
            size: vec2(viewport_size.width as i32, viewport_size.height as i32),
            cube: cube_mesh(),
            terrain: generate_terrain(vec2(0, 0), vec2(16, 16), vec3(1.0, 1.0, 1.0)),
            overlapping_tris: overlapping_tri_mesh(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = vec2(new_size.width as i32, new_size.height as i32);
        self.depth_buffer = Image::new(DepthF32::new(1.0), new_size.width, new_size.height);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size.x as f32 / self.size.y as f32
    }
}

pub fn render(renderer: &mut Renderer, pixel_buffer: &mut [u8], time: Duration, camera: &Camera) {
    let model_matrix1 = Matrix4::from_translation(vec3(1.5, 0.0, -3.0 * time.as_secs_f32().sin()))
        * Matrix4::from_angle_x(Rad(time.as_secs_f32().sin()))
        * Matrix4::from_angle_y(Rad(time.as_secs_f32()));
    let model_matrix2 = Matrix4::from_translation(vec3(-1.5, 0.0, -3.0))
        * Matrix4::from_angle_x(Rad(-(1.234 * time.as_secs_f32()).sin()))
        * Matrix4::from_angle_y(Rad(-0.8 * time.as_secs_f32()));
    let model_matrix3 = Matrix4::from_translation(vec3(0.0, 0.0, -3.0 * time.as_secs_f32().sin()))
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
            pipeline.run(
                &BasicUniforms::new(model_matrix, &view_matrix, &proj_matrix),
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

    let model_matrix = Matrix4::from_translation(vec3(0.0, 0.0, -5.0));
    for tri in renderer.overlapping_tris {
        pipeline.run(
            &BasicUniforms::new(&model_matrix, &view_matrix, &proj_matrix),
            (),
            &tri[0],
            &tri[1],
            &tri[2],
            renderer.size,
            &mut framebuffer,
            &mut depth_state,
        );
    }

    let model_matrix = Matrix4::from_translation(vec3(-8.0, -3.0, -8.0));
    for tri in renderer.terrain.mesh() {
        pipeline.run(
            &BasicUniforms::new(&model_matrix, &view_matrix, &proj_matrix),
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
