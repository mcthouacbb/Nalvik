mod material;
mod models;
mod render;

use utils::app::run_app;

use crate::{models::ModelPath, render::Renderer};

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
        println!("        Supported builtins: cube");
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

    let renderer = Renderer::new(&path);
    run_app(renderer);
}
