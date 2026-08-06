use crate::image::{
    format::ImageFormat,
    view::{ImageView, ImageViewMut},
};

pub mod format;
pub mod view;

pub struct Image<T: ImageFormat> {
    data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: ImageFormat> Image<T> {
    pub fn new(clear_value: T, width: u32, height: u32) -> Self {
        Self {
            data: vec![clear_value; (width * height) as usize],
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

    pub fn view(&self) -> ImageView<'_, T> {
        ImageView::new(&self.data, self.width, self.height)
    }

    pub fn view_mut(&mut self) -> ImageViewMut<'_, T> {
        ImageViewMut::new(&mut self.data, self.width, self.height)
    }

    pub fn clear(&mut self, value: T) {
        self.data.fill(value);
    }
}
