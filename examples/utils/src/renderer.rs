use crate::camera::Camera;

pub trait AppRenderer {
    fn resize(&mut self, new_width: u32, new_height: u32);
    fn render(&mut self, pixel_buffer: &mut [u8], camera: &Camera);
}
