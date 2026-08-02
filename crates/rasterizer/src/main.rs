mod app;
mod render;
mod util;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    if let Err(err) = event_loop.run_app(&mut app) {
        eprintln!("Event Loop Error: {}", err.to_string());
    }
}
