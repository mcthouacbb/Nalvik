use std::ptr::NonNull;

use crate::image::{format::ImageFormat, view::ImageViewMut};

pub struct TileMut<T: ImageFormat> {
    base_ptr: NonNull<T>,
    stride: u32,
    width: u32,
    height: u32,
}

impl<T: ImageFormat> TileMut<T> {
    pub fn new(
        image: &mut ImageViewMut<T>,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Self {
        let stride = image.width();
        let base_x = offset_x * width;
        let base_y = offset_y * height;

        debug_assert!(base_x < image.width());
        debug_assert!(base_y < image.height());

        let tile_width = width.min(image.width() - base_x);
        let tile_height = height.min(image.height() - base_y);

        Self {
            base_ptr: image.get_mut(base_x, base_y).into(),
            stride,
            width: tile_width,
            height: tile_height,
        }
    }

    pub unsafe fn get_ptr(&mut self, x: u32, y: u32) -> NonNull<T> {
        debug_assert!(x < self.width && y < self.height);

        unsafe { self.base_ptr.add((y * self.stride + x) as usize) }
    }
}

unsafe impl<T: ImageFormat> Send for TileMut<T> {}
