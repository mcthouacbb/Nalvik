use std::f32;

use cgmath::{
    Matrix3, Matrix4, Rad, Vector2, Vector3, Vector4, perspective, prelude::*, vec2, vec3,
};
use rasterizer::{
    DepthState, DepthTest, FilterMode, Image2d, Image2dView, Image2dViewMut,
    PERSPECTIVE_CORRECTION, Pipeline, Sampler2d, Uniforms, VertexOutput, VertexToFragment,
    format::{DepthF32, RgbaU8},
    unit_type_buf,
};
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera,
    material::Material,
    models::{self, ModelPath, VertexData},
};

#[derive(Clone, Copy, VertexToFragment)]
struct BasicVertexOutput {
    uv: Vector2<f32>,
    normal: Vector3<f32>,
}

struct BasicUniforms {
    mvp_matrix: Matrix4<f32>,
    normal_matrix: Matrix3<f32>,
}

struct TextureUniforms<'a> {
    texture: Image2dView<'a, RgbaU8>,
    sampler: Sampler2d,
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
    vertex_input: &VertexData,
    (uniforms, _, _, _): (&BasicUniforms, &TextureUniforms<'_>, &(), &()),
) -> VertexOutput<BasicVertexOutput> {
    let out_pos = uniforms.mvp_matrix * vertex_input.pos().extend(1.0);
    let out_normal = (uniforms.normal_matrix * vertex_input.normal()).normalize();
    VertexOutput {
        position: out_pos,
        data: BasicVertexOutput {
            uv: vertex_input.uv(),
            normal: out_normal,
        },
    }
}

fn fragment_shader(
    fragment_input: &BasicVertexOutput,
    (_, textures, _, _): (&BasicUniforms, &TextureUniforms<'_>, &(), &()),
) -> Vector4<f32> {
    // vec3(-0.4, -1, -0.5).normalized()
    const LIGHT_DIR: Vector3<f32> = vec3(-0.336860768, -0.84215192, -0.42107596);
    let brightness = 0.5 * (fragment_input.normal.normalize().dot(-LIGHT_DIR) + 1.0);
    let color = textures.sampler.sample(textures.texture, fragment_input.uv);
    (color.xyz() * brightness).extend(color.w)
}

pub struct Renderer {
    depth_buffer: Image2d<DepthF32>,
    size: Vector2<i32>,
    models: Vec<(Vec<[VertexData; 3]>, Material)>,
}

impl Renderer {
    pub fn new(viewport_size: PhysicalSize<u32>, model_path: &ModelPath) -> Self {
        Self {
            depth_buffer: Image2d::new(
                DepthF32::new(1.0),
                viewport_size.width,
                viewport_size.height,
            ),
            models: models::load_model(model_path),
            size: vec2(viewport_size.width as i32, viewport_size.height as i32),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = vec2(new_size.width as i32, new_size.height as i32);
        self.depth_buffer = Image2d::new(DepthF32::new(1.0), new_size.width, new_size.height);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size.x as f32 / self.size.y as f32
    }
}

pub fn render(renderer: &mut Renderer, pixel_buffer: &mut [u8], camera: &Camera) {
    let view_matrix = camera.view_matrix();
    let proj_matrix = PERSPECTIVE_CORRECTION
        * perspective(
            Rad(f32::consts::PI / 3.0),
            renderer.aspect_ratio(),
            0.1,
            200.0,
        );

    let pipeline = Pipeline::new(vertex_shader, fragment_shader);

    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[108, 182, 204, 0xFF]);
    }

    let mut framebuffer = Image2dViewMut::over_raw_bytes(
        pixel_buffer,
        renderer.size.x as u32,
        renderer.size.y as u32,
    );

    renderer.depth_buffer.clear(DepthF32::new(1.0));
    let mut depth_state =
        DepthState::CompareAndWrite(renderer.depth_buffer.view_mut(), DepthTest::Less);

    let mut uniform_buffer = Vec::new();
    let mut texture_buffer = Vec::new();

    uniform_buffer.push(BasicUniforms::new(
        &Matrix4::one(),
        &view_matrix,
        &proj_matrix,
    ));

    for (_, material) in &renderer.models {
        texture_buffer.push(TextureUniforms {
            texture: material.diffuse_texture().view(),
            sampler: Sampler2d::new(FilterMode::Nearest),
        });
    }

    let mut render_pass = pipeline.begin_render_pass(
        renderer.size,
        Uniforms::new(
            &uniform_buffer,
            &texture_buffer,
            unit_type_buf(),
            unit_type_buf(),
        ),
    );

    for (idx, (model, _)) in renderer.models.iter().enumerate() {
        for tri in model {
            pipeline.add_triangle(
                &mut render_pass,
                &tri[0],
                &tri[1],
                &tri[2],
                [0, 0 + idx as u32, 0, 0],
            )
        }
    }

    pipeline.run(&mut render_pass, &mut framebuffer, &mut depth_state);
}
