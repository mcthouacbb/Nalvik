use std::f32;

use cgmath::{Matrix4, Vector2, Vector3, Vector4, prelude::*, vec2, vec3, vec4};
use nalvik::{
    CullMode, DepthState, DepthTest, FilterMode, Image2dView, Image2dViewMut, Pipeline, Sampler2d,
    Uniforms, VertexOutput, VertexToFragment,
    format::{DepthF32, RgbaU8},
    unit_type_buf,
};
use utils::{camera::Camera, projection::oblique_projection};

use crate::{
    material::Material,
    render::{VertexData, scene::render_scene_objects},
    scene::Portal,
};

#[derive(Clone, Copy)]
struct PortalVertex {
    pos: Vector3<f32>,
}

impl PortalVertex {
    pub fn new(pos: Vector3<f32>) -> Self {
        Self { pos }
    }
}

#[derive(Clone, Copy, VertexToFragment)]
struct PortalVertexOutput {
    clip_pos: Vector4<f32>,
}

struct PortalUniforms<'a> {
    mvp_matrix: Matrix4<f32>,
    portal_texture: Image2dView<'a, RgbaU8>,
    sampler: Sampler2d,
}

fn vertex_shader(
    vertex_data: &PortalVertex,
    (uniforms, _, _, _): (&PortalUniforms, &(), &(), &()),
) -> VertexOutput<PortalVertexOutput> {
    let clip_pos = uniforms.mvp_matrix * vertex_data.pos.extend(1.0);
    VertexOutput {
        position: clip_pos,
        data: PortalVertexOutput { clip_pos },
    }
}

fn fragment_shader(
    fragment_input: &PortalVertexOutput,
    (uniforms, _, _, _): (&PortalUniforms, &(), &(), &()),
) -> Vector4<f32> {
    let uv = fragment_input.clip_pos.xy() / fragment_input.clip_pos.w * 0.5 + vec2(0.5, 0.5);
    uniforms.sampler.sample(uniforms.portal_texture, uv)
}

pub fn render_portal_cam(
    main_camera: &Camera,
    scene_objects: &Vec<(Vec<[VertexData; 3]>, Matrix4<f32>, Material)>,
    src_portal: &Portal,
    dst_portal: &mut Portal,
    aspect_ratio: f32,
    viewport_size: Vector2<i32>,
) {
    let portal_cam_view_matrix =
        main_camera.view_matrix() * dst_portal.transform * src_portal.transform.invert().unwrap();

    let mut plane = vec4(0.0, 0.0, -1.0, 0.0);
    let main_cam_offset =
        main_camera.position - (dst_portal.transform * vec4(0.0, 0.0, 0.0, 1.0)).xyz();
    if plane.dot(main_cam_offset.extend(1.0)) > 0.0 {
        plane = -plane;
    }

    let plane_transform = main_camera.view_matrix() * dst_portal.transform;
    let view_plane = plane_transform.transpose().invert().unwrap() * plane;

    let portal_cam_proj_matrix =
        oblique_projection(f32::consts::PI / 3.0, aspect_ratio, view_plane, 0.01);

    dst_portal
        .render_target
        .clear(RgbaU8::new(vec4(108, 182, 204, 0xFF)));

    dst_portal.depth_buffer.clear(DepthF32::new(1.0));

    render_scene_objects(
        dst_portal.render_target.view_mut(),
        dst_portal.depth_buffer.view_mut(),
        scene_objects,
        &portal_cam_view_matrix,
        &portal_cam_proj_matrix,
        viewport_size,
    );
}

pub fn render_portal_surface<'a>(
    mut color_buffer: Image2dViewMut<'a, RgbaU8>,
    depth_buffer: Image2dViewMut<'a, DepthF32>,
    portal: &Portal,
    view_matrix: &Matrix4<f32>,
    proj_matrix: &Matrix4<f32>,
    viewport_size: Vector2<i32>,
) {
    let surface = [
        [
            PortalVertex::new(vec3(-0.5, -0.5, 0.0)),
            PortalVertex::new(vec3(0.5, -0.5, 0.0)),
            PortalVertex::new(vec3(0.5, 0.5, 0.0)),
        ],
        [
            PortalVertex::new(vec3(0.5, 0.5, 0.0)),
            PortalVertex::new(vec3(-0.5, 0.5, 0.0)),
            PortalVertex::new(vec3(-0.5, -0.5, 0.0)),
        ],
    ];

    let uniform_buffer = [PortalUniforms {
        mvp_matrix: *proj_matrix * *view_matrix * portal.transform,
        portal_texture: portal.render_target.view(),
        sampler: Sampler2d::new(FilterMode::Nearest),
    }];

    let uniforms = Uniforms::new(
        &uniform_buffer,
        unit_type_buf(),
        unit_type_buf(),
        unit_type_buf(),
    );

    let pipeline = Pipeline::new(vertex_shader, fragment_shader);

    let mut render_pass = pipeline.begin_render_pass(viewport_size, uniforms);

    for tri in &surface {
        pipeline.add_triangle(&mut render_pass, &tri[0], &tri[1], &tri[2], [0, 0, 0, 0]);
    }

    pipeline.run(
        &mut render_pass,
        &mut color_buffer,
        &mut DepthState::CompareAndWrite(depth_buffer, DepthTest::Less),
        CullMode::RenderAll,
    );
}
