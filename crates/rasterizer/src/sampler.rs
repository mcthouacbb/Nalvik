use cgmath::{Vector2, Vector4};

use crate::image::{format::RgbaFormat, view::Image2dView};

#[derive(Clone, Copy)]
pub enum FilterMode {
    Nearest,
    Linear,
}

#[derive(Clone, Copy)]
pub struct Sampler2d {
    filter: FilterMode,
}

impl Sampler2d {
    pub fn new(filter: FilterMode) -> Self {
        Self { filter }
    }

    pub fn sample<'a, T: RgbaFormat>(
        &self,
        texture: Image2dView<'a, T>,
        uv: Vector2<f32>,
    ) -> Vector4<f32> {
        match self.filter {
            FilterMode::Nearest => {
                let texel_x = (uv.x.max(0.0) * texture.width() as f32) as u32;
                let texel_y = ((1.0 - uv.y).max(0.0) * texture.height() as f32) as u32;
                texture
                    .get(
                        texel_x.min(texture.width() - 1),
                        texel_y.min(texture.height() - 1),
                    )
                    .normalized()
            }
            FilterMode::Linear => {
                todo!()
            }
        }
    }
}
