use rust_rasterizer::core::canvas::Canvas;
use rust_rasterizer::rendering::obj::*;
use std::time::Instant;
use std::fs::File;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 1000;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let file = File::open("src/models/teapot.obj").unwrap();
    let mut model = Model::new(file);

    println!("Render started...");
    let now = Instant::now();

    render_wireframe(&model, &mut canvas);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

    Canvas::write_file(&mut canvas, "image");
}