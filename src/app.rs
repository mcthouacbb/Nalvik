use std::sync::Arc;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    minimized: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        Self {
            window: None,
            pixels: None,
            minimized: false,
        }
    }

    fn window(&self) -> &Window {
        self.window.as_deref().unwrap()
    }

    fn pixels(&self) -> &Pixels<'a> {
        self.pixels.as_ref().unwrap()
    }

    fn pixels_mut(&mut self) -> &mut Pixels<'a> {
        self.pixels.as_mut().unwrap()
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            self.window = Some(Arc::new(
                event_loop
                    .create_window(Window::default_attributes())
                    .unwrap(),
            ));
        }

        let size = self.window().inner_size();
        if self.pixels.is_none() {
            let surface_texture = SurfaceTexture::new(
                size.width,
                size.height,
                self.window.as_ref().unwrap().clone(),
            );
            self.pixels = Some(Pixels::new(size.width, size.height, surface_texture).unwrap());
        } else {
            self.pixels_mut()
                .resize_surface(size.width, size.height)
                .unwrap();
            self.pixels_mut()
                .resize_buffer(size.width, size.height)
                .unwrap();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        assert!(self.window().id() == window_id);
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.minimized = size.width == 0 && size.height == 0;
                if !self.minimized {
                    self.pixels_mut()
                        .resize_surface(size.width, size.height)
                        .unwrap();
                    self.pixels_mut()
                        .resize_buffer(size.width, size.height)
                        .unwrap();
                }

                self.window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if self.minimized {
                    return;
                }

                for pix in self.pixels_mut().frame_mut().chunks_exact_mut(4) {
                    pix.copy_from_slice(&[0, 0xFF, 0xFF, 0xFF]);
                }

                self.pixels().render().unwrap();

                self.window().request_redraw();
            }
            _ => (),
        }
    }
}
