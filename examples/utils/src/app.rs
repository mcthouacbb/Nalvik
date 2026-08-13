use std::{collections::HashSet, sync::Arc, time::Instant};

use cgmath::{vec2, vec3};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture, wgpu};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window, WindowId},
};

use crate::{camera::Camera, renderer::AppRenderer};

pub fn run_app<R: AppRenderer>(renderer: R) {
    const DEFAULT_SPEED: f32 = 12.0;
    run_app_with_camera_speed(renderer, DEFAULT_SPEED);
}

pub fn run_app_with_camera_speed<R: AppRenderer>(renderer: R, speed: f32) {
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(renderer, speed);
    if let Err(err) = event_loop.run_app(&mut app) {
        eprintln!("Event Loop Error: {}", err.to_string());
    }
}

pub struct App<'a, R: AppRenderer> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    renderer: R,
    size: PhysicalSize<u32>,
    minimized: bool,
    start_time: Instant,
    prev_time: Instant,
    avg_dt: f64,
    speed: f32,

    pressed_keys: HashSet<KeyCode>,
    camera: Camera,
    cursor_locked: bool,
}

impl<'a, R: AppRenderer> App<'a, R> {
    pub fn new(renderer: R, speed: f32) -> Self {
        let time = Instant::now();
        Self {
            window: None,
            pixels: None,
            renderer,
            size: PhysicalSize::new(0, 0),
            minimized: false,
            start_time: time,
            prev_time: time,
            avg_dt: 0.0,
            speed,
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

impl<'a, R: AppRenderer> ApplicationHandler for App<'a, R> {
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
            self.pixels = Some(
                PixelsBuilder::new(size.width, size.height, surface_texture)
                    .blend_state(wgpu::BlendState::REPLACE)
                    .alpha_mode(wgpu::CompositeAlphaMode::Opaque)
                    .build()
                    .unwrap(),
            );
        } else {
            self.pixels_mut()
                .resize_surface(size.width, size.height)
                .unwrap();
            self.pixels_mut()
                .resize_buffer(size.width, size.height)
                .unwrap();
        }

        self.renderer.resize(size.width, size.height);
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

                    self.renderer.resize(size.width, size.height);
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

                    if self.is_key_pressed(KeyCode::KeyE) {
                        self.camera.position.y += self.speed * dt.as_secs_f32();
                    }
                    if self.is_key_pressed(KeyCode::KeyQ) {
                        self.camera.position.y -= self.speed * dt.as_secs_f32();
                    }

                    if self.is_key_pressed(KeyCode::KeyW) {
                        self.camera.position.z -=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.cos();
                        self.camera.position.x -=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.sin();
                    }

                    if self.is_key_pressed(KeyCode::KeyS) {
                        self.camera.position.z +=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.cos();
                        self.camera.position.x +=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.sin();
                    }

                    if self.is_key_pressed(KeyCode::KeyD) {
                        self.camera.position.z -=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.sin();
                        self.camera.position.x +=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.cos();
                    }

                    if self.is_key_pressed(KeyCode::KeyA) {
                        self.camera.position.z +=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.sin();
                        self.camera.position.x -=
                            self.speed * dt.as_secs_f32() * self.camera.rotation.y.cos();
                    }

                    self.renderer
                        .render(self.pixels.as_mut().unwrap().frame_mut(), &self.camera);
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
