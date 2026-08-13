mod portal;
mod scene;

use std::f32;

use cgmath::{Vector2, Vector3, vec2, vec4};
use nalvik::{
    Image2d, Image2dViewMut,
    format::{DepthF32, RgbaU8},
};
use utils::{camera::Camera, projection::perspective_proj, renderer::AppRenderer};

use crate::{
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
    pub fn new() -> Self {
        Self {
            depth_buffer: Image2d::new(DepthF32::new(1.0), 0, 0),
            scene: scene(vec2(0, 0)),
            size: vec2(0, 0),
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

        self.scene.portal0.render_target = Image2d::new(
            RgbaU8::new(vec4(108, 182, 204, 0xFF)),
            new_width,
            new_height,
        );
        self.scene.portal0.depth_buffer = Image2d::new(DepthF32::new(1.0), new_width, new_height);

        self.scene.portal1.render_target = Image2d::new(
            RgbaU8::new(vec4(108, 182, 204, 0xFF)),
            new_width,
            new_height,
        );
        self.scene.portal1.depth_buffer = Image2d::new(DepthF32::new(1.0), new_width, new_height);
    }

    fn render(&mut self, pixel_buffer: &mut [u8], camera: &Camera) {
        let view_matrix = camera.view_matrix();
        let proj_matrix = perspective_proj(f32::consts::PI / 3.0, self.aspect_ratio(), 0.1, 200.0);

        // clear buffer
        for pix in pixel_buffer.chunks_exact_mut(4) {
            pix.copy_from_slice(&[108, 182, 204, 0xFF]);
        }

        self.depth_buffer.clear(DepthF32::new(1.0));

        render_scene_objects(
            Image2dViewMut::over_raw_bytes(pixel_buffer, self.size.x as u32, self.size.y as u32),
            self.depth_buffer.view_mut(),
            &self.scene.objects,
            &view_matrix,
            &proj_matrix,
            self.size,
        );

        let aspect_ratio = self.aspect_ratio();
        render_portal_cam(
            camera,
            &self.scene.objects,
            &self.scene.portal1,
            &mut self.scene.portal0,
            aspect_ratio,
            self.size,
        );

        render_portal_cam(
            camera,
            &self.scene.objects,
            &self.scene.portal0,
            &mut self.scene.portal1,
            aspect_ratio,
            self.size,
        );

        render_portal_surface(
            Image2dViewMut::over_raw_bytes(pixel_buffer, self.size.x as u32, self.size.y as u32),
            self.depth_buffer.view_mut(),
            &self.scene.portal0,
            &view_matrix,
            &proj_matrix,
            self.size,
        );

        render_portal_surface(
            Image2dViewMut::over_raw_bytes(pixel_buffer, self.size.x as u32, self.size.y as u32),
            self.depth_buffer.view_mut(),
            &self.scene.portal1,
            &view_matrix,
            &proj_matrix,
            self.size,
        );
    }
}
