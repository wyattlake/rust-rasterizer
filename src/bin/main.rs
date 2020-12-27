use rust_rasterizer::core::canvas::Canvas;
use rust_rasterizer::core::color::Color;
use rust_rasterizer::core::line::*;
use std::time::Instant;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 200;
    const HEIGHT: usize = 200;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    println!("Render started...");
    let now = Instant::now();

    draw_line(0, 0, 100, 199, &mut canvas, &Color(1.0, 1.0, 1.0));

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

    Canvas::write_file(&mut canvas, "image");
}