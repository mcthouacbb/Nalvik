use std::path::Path;

use cgmath::{Vector4, vec4};
use nalvik::{
    Image2d, ImageLoadError,
    format::{RgbaF32, RgbaU8},
};

#[derive(Clone)]
pub struct Material {
    diffuse_texture: Image2d<RgbaU8>,
}

impl Material {
    pub fn try_load_from_file(file: impl AsRef<Path>) -> Result<Self, ImageLoadError> {
        let diffuse_texture = Image2d::load_from_file(file)?;
        Ok(Self { diffuse_texture })
    }

    pub fn solid_color(color: Vector4<f32>) -> Self {
        Self {
            diffuse_texture: Image2d::new(RgbaU8::from(RgbaF32::new(color)), 1, 1),
        }
    }

    pub fn debug_material() -> Self {
        Self::solid_color(vec4(1.0, 0.0, 1.0, 1.0))
    }

    pub fn diffuse_texture(&self) -> &Image2d<RgbaU8> {
        &self.diffuse_texture
    }
}
