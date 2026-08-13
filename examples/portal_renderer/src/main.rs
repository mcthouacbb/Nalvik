use utils::app::run_app;

use crate::render::Renderer;

mod material;
mod render;
mod scene;

fn main() {
    run_app(Renderer::new());
}
