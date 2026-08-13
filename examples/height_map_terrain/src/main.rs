mod render;
mod terrain;

use utils::app::run_app_with_camera_speed;

use crate::render::Renderer;

fn help_message() {
    println!("Height Map Terrain");
    println!("    Loads .obj models and displays them in an interactive window");
    println!();
    println!("USAGE");
    println!("    model_loader [OPTIONS]");
    println!();
    println!("OPTIONS");
    println!("    -h, --help");
    println!("        Displays this help message");
    println!();
    println!("    --render-distance, -r <distance>    [default: 8]");
    println!(
        "        Specifies how far chunks can be before they stop rendering. Must be a nonnegative integer"
    );
    println!();
    println!("    --speed, -s <distance>    [default: 12.0]");
    println!("        Specifies how fast the camera moves. Must be a positive float");
}

struct Args {
    help: bool,
    render_dist: u32,
    speed: f32,
}

fn parse_single_arg(args: &[String], index: &mut usize, parsed_args: &mut Args) -> Result<(), ()> {
    if *index < args.len() {
        let arg = &args[*index];
        match arg.as_str() {
            "-h" | "--help" => {
                parsed_args.help = true;
                Ok(())
            }
            "-r" | "--render-dist" => {
                *index += 1;
                if *index < args.len() {
                    match args[*index].parse::<u32>() {
                        Ok(render_dist) => {
                            *index += 1;
                            parsed_args.render_dist = render_dist;
                            Ok(())
                        }
                        Err(_) => {
                            eprintln!(
                                "Invalid render distance. Please specify a nonnegative integer."
                            );
                            Err(())
                        }
                    }
                } else {
                    eprintln!(
                        "No render distance specified. -r/--render-distance requires a nonnegative integer."
                    );
                    Err(())
                }
            }
            "-s" | "--speed" => {
                *index += 1;
                if *index < args.len() {
                    match args[*index].parse::<f32>() {
                        Ok(speed) => {
                            *index += 1;
                            parsed_args.speed = speed;
                            Ok(())
                        }
                        Err(_) => {
                            eprintln!(
                                "Invalid speed. Please specify a positive floating point number."
                            );
                            Err(())
                        }
                    }
                } else {
                    eprintln!(
                        "No speed specified. -s/--speed requires a positive floating point number."
                    );
                    Err(())
                }
            }
            _ => {
                eprintln!("Unrecognized token {}", args[*index]);
                Err(())
            }
        }
    } else {
        Ok(())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut parsed_args = Args {
        help: false,
        render_dist: 8,
        speed: 12.0,
    };
    let mut i = 1;
    while i < args.len() {
        if parse_single_arg(&args, &mut i, &mut parsed_args).is_err() {
            return;
        }
    }

    if parsed_args.help {
        help_message();
        return;
    }

    let renderer = Renderer::new(parsed_args.render_dist);
    run_app_with_camera_speed(renderer, parsed_args.speed);
}
