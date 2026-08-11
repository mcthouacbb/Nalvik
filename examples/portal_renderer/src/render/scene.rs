use cgmath::{Matrix3, Matrix4, Vector2, Vector3, Vector4, prelude::*, vec3};
use nalvik::{
    CullMode, DepthState, DepthTest, Image2dViewMut, Pipeline, Uniforms, VertexOutput,
    VertexToFragment,
    format::{DepthF32, RgbaU8},
    unit_type_buf,
};

use crate::{material::Material, render::VertexData};

#[derive(Clone, Copy, VertexToFragment)]
struct BasicVertexOutput {
    world_pos: Vector3<f32>,
    normal: Vector3<f32>,
}

struct BasicUniforms {
    model_matrix: Matrix4<f32>,
    mvp_matrix: Matrix4<f32>,
    normal_matrix: Matrix3<f32>,
    color0: Vector3<f32>,
    color1: Vector3<f32>,
}

impl BasicUniforms {
    fn new(
        model_matrix: &Matrix4<f32>,
        view_matrix: &Matrix4<f32>,
        proj_matrix: &Matrix4<f32>,
        material: &Material,
    ) -> Self {
        Self {
            model_matrix: *model_matrix,
            mvp_matrix: proj_matrix * view_matrix * model_matrix,
            normal_matrix: Matrix3::from_cols(
                model_matrix.x.xyz(),
                model_matrix.y.xyz(),
                model_matrix.z.xyz(),
            )
            .invert()
            .unwrap()
            .transpose(),
            color0: material.color0(),
            color1: material.color1(),
        }
    }
}

fn vertex_shader(
    vertex_input: &VertexData,
    (uniforms, _, _, _): (&BasicUniforms, &(), &(), &()),
) -> VertexOutput<BasicVertexOutput> {
    let out_pos = uniforms.mvp_matrix * vertex_input.pos().extend(1.0);
    let out_normal = (uniforms.normal_matrix * vertex_input.normal()).normalize();
    VertexOutput {
        position: out_pos,
        data: BasicVertexOutput {
            world_pos: (uniforms.model_matrix * vertex_input.pos().extend(1.0)).xyz(),
            normal: out_normal,
        },
    }
}

fn fragment_shader(
    fragment_input: &BasicVertexOutput,
    (uniforms, _, _, _): (&BasicUniforms, &(), &(), &()),
) -> Vector4<f32> {
    // vec3(-0.4, -1, -0.5).normalized()
    const LIGHT_DIR: Vector3<f32> = vec3(-0.336860768, -0.84215192, -0.42107596);
    let brightness = 0.4 * (fragment_input.normal.normalize().dot(-LIGHT_DIR) + 1.5);

    let det = (fragment_input.world_pos.x * 1.68 + 0.31).floor() as i32
        + (fragment_input.world_pos.y * 1.68 - 0.41).floor() as i32
        + (fragment_input.world_pos.z * 1.68 + 0.12).floor() as i32;

    let color = if det % 2 == 0 {
        uniforms.color0
    } else {
        uniforms.color1
    };

    (color.xyz() * brightness).extend(1.0)
}

pub fn render_scene_objects<'a>(
    mut color_buffer: Image2dViewMut<'a, RgbaU8>,
    depth_buffer: Image2dViewMut<'a, DepthF32>,
    scene_objects: &Vec<(Vec<[VertexData; 3]>, Matrix4<f32>, Material)>,
    view_matrix: &Matrix4<f32>,
    proj_matrix: &Matrix4<f32>,
    viewport_size: Vector2<i32>,
) {
    let mut uniform_buffer = Vec::new();

    for (_, transform, material) in scene_objects {
        uniform_buffer.push(BasicUniforms::new(
            transform,
            view_matrix,
            proj_matrix,
            material,
        ));
    }

    let pipeline = Pipeline::new(vertex_shader, fragment_shader);

    let mut render_pass = pipeline.begin_render_pass(
        viewport_size,
        Uniforms::new(
            &uniform_buffer,
            unit_type_buf(),
            unit_type_buf(),
            unit_type_buf(),
        ),
    );

    for (idx, (model, _, _)) in scene_objects.iter().enumerate() {
        for tri in model {
            pipeline.add_triangle(
                &mut render_pass,
                &tri[0],
                &tri[1],
                &tri[2],
                [idx as u32, 0, 0, 0],
            )
        }
    }

    pipeline.run(
        &mut render_pass,
        &mut color_buffer,
        &mut DepthState::CompareAndWrite(depth_buffer, DepthTest::Less),
        CullMode::OnlyRenderCCW,
    );
}
