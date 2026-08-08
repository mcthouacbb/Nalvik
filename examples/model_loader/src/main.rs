mod app;
mod camera;
mod material;
mod models;
mod render;

use winit::event_loop::{ControlFlow, EventLoop};

use crate::{app::App, models::ModelPath};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 || args[1] == "help" || args[1] == "-h" || args[1] == "--help" {
        println!("Model Loader");
        println!("    Loads .obj models and displays them in an interactive window");
        println!();
        println!("USAGE");
        println!("    model_loader [OPTIONS] <model_path>");
        println!();
        println!("OPTIONS");
        println!("    -h, --help");
        println!("        Displays this help message");
        println!();
        println!("    -b, --builtin");
        println!("        Use a builtin model instead of finding the model in the filesystem");
        return;
    }
    let path = if args[1] == "--builtin" || args[1] == "-b" {
        if args.len() < 3 {
            eprintln!("ERROR: No model specified");
            return;
        }
        ModelPath::Builtin(args[2].clone())
    } else {
        ModelPath::File(args[1].clone())
    };

    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new(path);
    if let Err(err) = event_loop.run_app(&mut app) {
        eprintln!("Event Loop Error: {}", err.to_string());
    }
}
