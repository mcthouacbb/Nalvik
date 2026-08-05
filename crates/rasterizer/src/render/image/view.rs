use crate::render::image::format::ImageFormat;

#[derive(Clone)]
pub struct ImageView<'a, T: ImageFormat> {
    data: &'a [T],
    width: u32,
    height: u32,
}

impl<'a, T: ImageFormat> ImageView<'a, T> {
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

pub struct ImageViewMut<'a, T: ImageFormat> {
    data: &'a mut [T],
    width: u32,
    height: u32,
}

impl<'a, T: ImageFormat> ImageViewMut<'a, T> {
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
