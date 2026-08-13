use bytemuck::{AnyBitPattern, NoUninit};
use cgmath::{Vector4, vec4};

pub trait ImageFormat: Copy + Sync + Send {
    const NORMALIZED: bool;
}

pub trait RgbaFormat: ImageFormat + From<RgbaF32> {
    fn normalized(&self) -> Vector4<f32>;
}

pub trait DepthFormat: ImageFormat + From<f32> {
    fn compare_less(a: &Self, b: &Self) -> bool;
    fn compare_greater(a: &Self, b: &Self) -> bool;
    fn compare_less_equal(a: &Self, b: &Self) -> bool;
    fn compare_greater_equal(a: &Self, b: &Self) -> bool;
    fn compare_equal(a: &Self, b: &Self) -> bool;
}

#[derive(Clone, Copy)]
pub struct RgbaF32 {
    rgba: [f32; 4],
}

impl RgbaF32 {
    pub fn new(rgba: Vector4<f32>) -> Self {
        Self {
            rgba: [rgba[0], rgba[1], rgba[2], rgba[3]],
        }
    }
}

impl ImageFormat for RgbaF32 {
    const NORMALIZED: bool = false;
}

impl RgbaFormat for RgbaF32 {
    fn normalized(&self) -> Vector4<f32> {
        vec4(self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3])
    }
}

#[repr(C)]
#[derive(Clone, Copy, NoUninit, AnyBitPattern)]
pub struct RgbaU8 {
    rgba: [u8; 4],
}

impl RgbaU8 {
    pub fn new(rgba: Vector4<u8>) -> Self {
        Self {
            rgba: [rgba[0], rgba[1], rgba[2], rgba[3]],
        }
    }
}

impl ImageFormat for RgbaU8 {
    const NORMALIZED: bool = true;
}

impl RgbaFormat for RgbaU8 {
    fn normalized(&self) -> Vector4<f32> {
        const INV_255: f32 = 1.0 / 255.0;
        vec4(
            self.rgba[0] as f32 * INV_255,
            self.rgba[1] as f32 * INV_255,
            self.rgba[2] as f32 * INV_255,
            self.rgba[3] as f32 * INV_255,
        )
    }
}

impl From<RgbaF32> for RgbaU8 {
    fn from(value: RgbaF32) -> Self {
        Self {
            rgba: [
                (value.rgba[0] * 255.0 + 0.5) as u8,
                (value.rgba[1] * 255.0 + 0.5) as u8,
                (value.rgba[2] * 255.0 + 0.5) as u8,
                (value.rgba[3] * 255.0 + 0.5) as u8,
            ],
        }
    }
}

#[derive(Clone, Copy)]
pub struct DepthF32 {
    depth: f32,
}

impl DepthF32 {
    pub fn new(depth: f32) -> Self {
        Self { depth }
    }
}

impl From<f32> for DepthF32 {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl ImageFormat for DepthF32 {
    const NORMALIZED: bool = true;
}

impl DepthFormat for DepthF32 {
    fn compare_less(a: &Self, b: &Self) -> bool {
        a.depth < b.depth
    }

    fn compare_greater(a: &Self, b: &Self) -> bool {
        a.depth > b.depth
    }

    fn compare_less_equal(a: &Self, b: &Self) -> bool {
        a.depth <= b.depth
    }

    fn compare_greater_equal(a: &Self, b: &Self) -> bool {
        a.depth >= b.depth
    }

    fn compare_equal(a: &Self, b: &Self) -> bool {
        a.depth == b.depth
    }
}
