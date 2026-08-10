mod portal;
mod scene;

use std::f32;

use cgmath::{Rad, Vector2, Vector3, perspective, vec2, vec4};
use rasterizer::{
    Image2d, Image2dViewMut, PERSPECTIVE_CORRECTION,
    format::{DepthF32, RgbaU8},
};
use winit::dpi::PhysicalSize;

use crate::{
    camera::Camera,
    render::{
        portal::{render_portal_cam, render_portal_surface},
        scene::render_scene_objects,
    },
    scene::{Scene, scene},
};

#[derive(Clone, Copy)]
pub struct VertexData {
    pos: Vector3<f32>,
    normal: Vector3<f32>,
}

impl VertexData {
    pub fn new(pos: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, normal }
    }

    pub fn pos(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }

    pub fn flip_normal(&mut self) {
        self.normal *= -1.0;
    }
}

pub struct Renderer {
    depth_buffer: Image2d<DepthF32>,
    size: Vector2<i32>,
    scene: Scene,
}

impl Renderer {
    pub fn new(viewport_size: PhysicalSize<u32>) -> Self {
        let size = vec2(viewport_size.width as i32, viewport_size.height as i32);
        Self {
            depth_buffer: Image2d::new(
                DepthF32::new(1.0),
                viewport_size.width,
                viewport_size.height,
            ),
            scene: scene(size),
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = vec2(new_size.width as i32, new_size.height as i32);
        self.depth_buffer = Image2d::new(DepthF32::new(1.0), new_size.width, new_size.height);

        self.scene.portal0.render_target = Image2d::new(
            RgbaU8::new(vec4(108, 182, 204, 0xFF)),
            new_size.width,
            new_size.height,
        );
        self.scene.portal0.depth_buffer =
            Image2d::new(DepthF32::new(1.0), new_size.width, new_size.height);

        self.scene.portal1.render_target = Image2d::new(
            RgbaU8::new(vec4(108, 182, 204, 0xFF)),
            new_size.width,
            new_size.height,
        );
        self.scene.portal1.depth_buffer =
            Image2d::new(DepthF32::new(1.0), new_size.width, new_size.height);
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

    // clear buffer
    for pix in pixel_buffer.chunks_exact_mut(4) {
        pix.copy_from_slice(&[108, 182, 204, 0xFF]);
    }

    renderer.depth_buffer.clear(DepthF32::new(1.0));

    render_scene_objects(
        Image2dViewMut::over_raw_bytes(
            pixel_buffer,
            renderer.size.x as u32,
            renderer.size.y as u32,
        ),
        renderer.depth_buffer.view_mut(),
        &renderer.scene.objects,
        &view_matrix,
        &proj_matrix,
        renderer.size,
    );

    let aspect_ratio = renderer.aspect_ratio();
    render_portal_cam(
        camera,
        &renderer.scene.objects,
        &renderer.scene.portal1,
        &mut renderer.scene.portal0,
        aspect_ratio,
        renderer.size,
    );

    render_portal_cam(
        camera,
        &renderer.scene.objects,
        &renderer.scene.portal0,
        &mut renderer.scene.portal1,
        aspect_ratio,
        renderer.size,
    );

    render_portal_surface(
        Image2dViewMut::over_raw_bytes(
            pixel_buffer,
            renderer.size.x as u32,
            renderer.size.y as u32,
        ),
        renderer.depth_buffer.view_mut(),
        &renderer.scene.portal0,
        &view_matrix,
        &proj_matrix,
        renderer.size,
    );

    render_portal_surface(
        Image2dViewMut::over_raw_bytes(
            pixel_buffer,
            renderer.size.x as u32,
            renderer.size.y as u32,
        ),
        renderer.depth_buffer.view_mut(),
        &renderer.scene.portal1,
        &view_matrix,
        &proj_matrix,
        renderer.size,
    );
}
