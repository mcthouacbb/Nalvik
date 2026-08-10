use std::f32;

use cgmath::{Matrix4, Rad, Vector2, Vector3, Vector4, perspective, prelude::*, vec2, vec3, vec4};
use rasterizer::{
    DepthState, DepthTest, FilterMode, Image2dView, Image2dViewMut, PERSPECTIVE_CORRECTION,
    Pipeline, Sampler2d, Uniforms, VertexOutput, VertexToFragment,
    format::{DepthF32, RgbaU8},
    unit_type_buf,
};

use crate::{
    camera::Camera,
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

fn oblique_projection(
    fovy: Rad<f32>,
    aspect: f32,
    near_plane: Vector4<f32>,
    far_k: f32,
) -> Matrix4<f32> {
    let two: f32 = 2.0;
    let f = Rad::cot(fovy / two);

    let c0r0 = f / aspect;
    let c0r1 = f32::zero();
    let c0r2 = far_k * near_plane.x;
    let c0r3 = f32::zero();

    let c1r0 = f32::zero();
    let c1r1 = f;
    let c1r2 = far_k * near_plane.y;
    let c1r3 = f32::zero();

    let c2r0 = f32::zero();
    let c2r1 = f32::zero();
    let c2r2 = far_k * near_plane.z;
    let c2r3 = -f32::one();

    let c3r0 = f32::zero();
    let c3r1 = f32::zero();
    let c3r2 = far_k * near_plane.w;
    let c3r3 = f32::zero();

    #[cfg_attr(rustfmt, rustfmt_skip)]
        Matrix4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
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
        oblique_projection(Rad(f32::consts::PI / 3.0), aspect_ratio, view_plane, 0.01);

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
        // +z face
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
        // -z face
        [
            PortalVertex::new(vec3(0.5, -0.5, 0.0)),
            PortalVertex::new(vec3(-0.5, -0.5, 0.0)),
            PortalVertex::new(vec3(-0.5, 0.5, 0.0)),
        ],
        [
            PortalVertex::new(vec3(-0.5, 0.5, 0.0)),
            PortalVertex::new(vec3(0.5, 0.5, 0.0)),
            PortalVertex::new(vec3(0.5, -0.5, 0.0)),
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

    for tri in surface {
        pipeline.add_triangle(&mut render_pass, &tri[0], &tri[1], &tri[2], [0, 0, 0, 0]);
    }

    pipeline.run(
        &mut render_pass,
        &mut color_buffer,
        &mut DepthState::CompareAndWrite(depth_buffer, DepthTest::Less),
    );
}
