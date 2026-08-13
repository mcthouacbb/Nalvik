use std::f32;

use cgmath::{Matrix, Matrix3, Matrix4, Vector2, Vector3, Vector4, prelude::*, vec2, vec3};
use nalvik::{
    CullMode, DepthState, DepthTest, Image2d, Image2dViewMut, Pipeline, Uniforms, VertexOutput,
    VertexToFragment, format::DepthF32, unit_type_buf,
};
use utils::{camera::Camera, projection::perspective_proj, renderer::AppRenderer};

use crate::terrain::manager::{CHUNK_SIZE, ChunkManager};

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
    (uniforms, _, _, _): (&BasicUniforms, &(), &(), &()),
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

fn fragment_shader(
    fragment_input: &BasicVertexOutput,
    _: (&BasicUniforms, &(), &(), &()),
) -> Vector4<f32> {
    // vec3(-0.4, -1, -0.5).normalized()
    const LIGHT_DIR: Vector3<f32> = vec3(-0.336860768, -0.84215192, -0.42107596);
    let brightness = 0.5 * (fragment_input.normal.normalize().dot(-LIGHT_DIR) + 1.0);
    (fragment_input.color * brightness).extend(1.0)
}

pub struct Renderer {
    depth_buffer: Image2d<DepthF32>,
    size: Vector2<i32>,
    chunk_manager: ChunkManager,
}

impl Renderer {
    pub fn new(render_distance: u32) -> Self {
        Self {
            depth_buffer: Image2d::new(DepthF32::new(1.0), 0, 0),
            size: vec2(0 as i32, 0 as i32),
            chunk_manager: ChunkManager::new(render_distance),
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size.x as f32 / self.size.y as f32
    }
}

impl AppRenderer for Renderer {
    fn resize(&mut self, new_width: u32, new_height: u32) {
        self.size = vec2(new_width as i32, new_height as i32);
        self.depth_buffer = Image2d::new(DepthF32::new(1.0), new_width, new_height);
    }

    fn render(&mut self, pixel_buffer: &mut [u8], camera: &Camera) {
        let view_matrix = camera.view_matrix();
        let proj_matrix = perspective_proj(
            f32::consts::PI / 3.0,
            self.aspect_ratio(),
            0.25,
            2.0 * self.chunk_manager.render_distance() as f32
                * CHUNK_SIZE.cast::<f32>().unwrap().magnitude()
                + 10.0,
        );

        self.chunk_manager.update_chunks(camera.position.xz());

        let pipeline = Pipeline::new(vertex_shader, fragment_shader);

        // clear buffer
        for pix in pixel_buffer.chunks_exact_mut(4) {
            pix.copy_from_slice(&[108, 182, 204, 0xFF]);
        }

        let mut framebuffer =
            Image2dViewMut::over_raw_bytes(pixel_buffer, self.size.x as u32, self.size.y as u32);

        self.depth_buffer.clear(DepthF32::new(1.0));
        let mut depth_state =
            DepthState::CompareAndWrite(self.depth_buffer.view_mut(), DepthTest::Less);

        let mut uniform_buffer = Vec::new();

        for chunk in &self.chunk_manager.get_active_chunks(camera.position.xz()) {
            let model_matrix = Matrix4::from_translation(vec3(
                chunk.base_pos().x as f32,
                -3.0,
                chunk.base_pos().y as f32,
            ));
            uniform_buffer.push(BasicUniforms::new(
                &model_matrix,
                &view_matrix,
                &proj_matrix,
            ));
        }

        let mut render_pass = pipeline.begin_render_pass(
            self.size,
            Uniforms::new(
                &uniform_buffer,
                unit_type_buf(),
                unit_type_buf(),
                unit_type_buf(),
            ),
        );

        for (idx, &chunk) in (&self.chunk_manager.get_active_chunks(camera.position.xz()))
            .iter()
            .enumerate()
        {
            for tri in chunk.mesh() {
                pipeline.add_triangle(
                    &mut render_pass,
                    &tri[0],
                    &tri[1],
                    &tri[2],
                    [idx as u32, 0, 0, 0],
                );
            }
        }

        pipeline.run(
            &mut render_pass,
            &mut framebuffer,
            &mut depth_state,
            CullMode::RenderOnlyCCW,
        );
    }
}
