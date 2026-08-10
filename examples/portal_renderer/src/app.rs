use std::{collections::HashSet, sync::Arc, time::Instant};

use cgmath::{vec2, vec3};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, MouseButton, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

use crate::{
    camera::Camera,
    render::{Renderer, render},
};

pub struct App<'a> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    renderer: Option<Renderer>,
    size: PhysicalSize<u32>,
    minimized: bool,
    start_time: Instant,
    prev_time: Instant,
    avg_dt: f64,

    pressed_keys: HashSet<KeyCode>,
    camera: Camera,
    cursor_locked: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let time = Instant::now();
        Self {
            window: None,
            pixels: None,
            renderer: None,
            size: PhysicalSize::new(0, 0),
            minimized: false,
            start_time: time,
            prev_time: time,
            avg_dt: 0.0,
            pressed_keys: HashSet::new(),
            camera: Camera::new(vec3(0.0, 0.0, 0.0), vec2(0.0, 0.0)),
            cursor_locked: false,
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

    fn renderer(&self) -> &Renderer {
        self.renderer.as_ref().unwrap()
    }

    fn renderer_mut(&mut self) -> &mut Renderer {
        self.renderer.as_mut().unwrap()
    }

    fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.pressed_keys.contains(&key_code)
    }

    fn lock_cursor(&mut self) {
        let _ = self.window().set_cursor_grab(CursorGrabMode::Locked);
        self.window().set_cursor_visible(false);
        self.cursor_locked = true;
    }

    fn unlock_cursor(&mut self) {
        let _ = self.window().set_cursor_grab(CursorGrabMode::None);
        self.window().set_cursor_visible(true);
        self.cursor_locked = false;
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
        self.size = size;

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

        if self.renderer.is_none() {
            self.renderer = Some(Renderer::new(size));
        } else {
            self.renderer_mut().resize(size);
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
                self.size = size;
                self.minimized = size.width == 0 && size.height == 0;

                if !self.minimized {
                    self.pixels_mut()
                        .resize_surface(size.width, size.height)
                        .unwrap();
                    self.pixels_mut()
                        .resize_buffer(size.width, size.height)
                        .unwrap();

                    self.renderer_mut().resize(size);
                }

                self.window().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                if self.minimized {
                    return;
                }

                if !self.minimized {
                    let curr_time = Instant::now();
                    let time = curr_time - self.start_time;
                    let prev_time = self.prev_time - self.start_time;
                    let dt = curr_time - self.prev_time;
                    self.prev_time = curr_time;

                    self.avg_dt = 0.9 * self.avg_dt + 0.1 * dt.as_secs_f64();
                    if time.as_secs_f32().ceil() != prev_time.as_secs_f32().ceil() {
                        println!("FPS: {}", (10.0 / self.avg_dt).round() / 10.0);
                    }

                    const SPEED: f32 = 4.0;

                    if self.is_key_pressed(KeyCode::KeyE) {
                        self.camera.position.y += SPEED * dt.as_secs_f32();
                    }
                    if self.is_key_pressed(KeyCode::KeyQ) {
                        self.camera.position.y -= SPEED * dt.as_secs_f32();
                    }

                    if self.is_key_pressed(KeyCode::KeyW) {
                        self.camera.position.z -=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.cos();
                        self.camera.position.x -=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.sin();
                    }

                    if self.is_key_pressed(KeyCode::KeyS) {
                        self.camera.position.z +=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.cos();
                        self.camera.position.x +=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.sin();
                    }

                    if self.is_key_pressed(KeyCode::KeyD) {
                        self.camera.position.z -=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.sin();
                        self.camera.position.x +=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.cos();
                    }

                    if self.is_key_pressed(KeyCode::KeyA) {
                        self.camera.position.z +=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.sin();
                        self.camera.position.x -=
                            SPEED * dt.as_secs_f32() * self.camera.rotation.y.cos();
                    }

                    render(
                        self.renderer.as_mut().unwrap(),
                        self.pixels.as_mut().unwrap().frame_mut(),
                        &self.camera,
                    );
                }

                self.pixels().render().unwrap();
                self.window().request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    // ignore unidentified keys
                    if event.state == ElementState::Pressed {
                        self.pressed_keys.insert(key_code);
                    } else {
                        self.pressed_keys.remove(&key_code);
                    }

                    if key_code == KeyCode::Escape {
                        self.unlock_cursor();
                    }
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                if button == MouseButton::Left && state.is_pressed() {
                    self.lock_cursor();
                }
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                if self.cursor_locked {
                    self.camera.rotation.y -= 0.002 * delta.0 as f32;
                    self.camera.rotation.x -= 0.002 * delta.1 as f32;
                }
            }
            _ => (),
        }
    }
}
