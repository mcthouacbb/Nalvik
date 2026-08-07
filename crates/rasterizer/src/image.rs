use std::{io, path::Path};

use image::{ImageError, ImageReader};

use crate::{
    format::RgbaU8,
    image::{
        format::ImageFormat,
        view::{Image2dView, Image2dViewMut},
    },
};

pub mod format;
pub mod view;

pub struct Image2d<T: ImageFormat> {
    data: Vec<T>,
    width: u32,
    height: u32,
}

impl<T: ImageFormat> Image2d<T> {
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

    pub fn view(&self) -> Image2dView<'_, T> {
        Image2dView::new(&self.data, self.width, self.height)
    }

    pub fn view_mut(&mut self) -> Image2dViewMut<'_, T> {
        Image2dViewMut::new(&mut self.data, self.width, self.height)
    }

    pub fn clear(&mut self, value: T) {
        self.data.fill(value);
    }
}

#[derive(Debug)]
pub enum ImageLoadError {
    Io(io::Error),
    Image(ImageError),
}

impl From<io::Error> for ImageLoadError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ImageError> for ImageLoadError {
    fn from(value: ImageError) -> Self {
        Self::Image(value)
    }
}

impl Image2d<RgbaU8> {
    pub fn load_from_file(file: impl AsRef<Path>) -> Result<Self, ImageLoadError> {
        let reader = ImageReader::open(file)?;
        let image = reader.decode()?;
        let width = image.width();
        let height = image.height();
        let raw_data = image.into_rgba8().into_raw();
        Ok(Self {
            data: bytemuck::allocation::cast_vec(raw_data),
            width,
            height,
        })
    }
}
