use crate::{format::RgbaU8, image::format::ImageFormat};

#[derive(Clone, Copy)]
pub struct Image2dView<'a, T: ImageFormat> {
    data: &'a [T],
    width: u32,
    height: u32,
}

impl<'a, T: ImageFormat> Image2dView<'a, T> {
    pub fn new(data: &'a [T], width: u32, height: u32) -> Self {
        assert!(data.len() as u32 == width * height);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> &T {
        &self.data[(y * self.width + x) as usize]
    }
}

impl<'a> Image2dView<'a, RgbaU8> {
    pub fn over_raw_bytes(bytes: &'a [u8], width: u32, height: u32) -> Self {
        Self {
            data: bytemuck::cast_slice(bytes),
            width,
            height,
        }
    }
}

pub struct Image2dViewMut<'a, T: ImageFormat> {
    data: &'a mut [T],
    width: u32,
    height: u32,
}

impl<'a, T: ImageFormat> Image2dViewMut<'a, T> {
    pub fn new(data: &'a mut [T], width: u32, height: u32) -> Self {
        assert!(data.len() as u32 == width * height);
        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get(&self, x: u32, y: u32) -> &T {
        &self.data[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut T {
        &mut self.data[(y * self.width + x) as usize]
    }
}

impl<'a> Image2dViewMut<'a, RgbaU8> {
    pub fn over_raw_bytes(bytes: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            data: bytemuck::cast_slice_mut(bytes),
            width,
            height,
        }
    }
}
