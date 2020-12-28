use rust_rasterizer::core::canvas::Canvas;
use rust_rasterizer::rendering::obj::*;
use image::io::Reader as ImageReader;
use std::time::Instant;
use std::fs::File;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let file = File::open("src/models/model.obj").unwrap();
    let mut model = Model::new(file);
    let img = ImageReader::open("src/models/texture.tga").unwrap().decode().unwrap();
    model.load_texture(img);

    println!("Render started...");
    let now = Instant::now();

    render_model(&model, &mut canvas);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );

    Canvas::write_file(&mut canvas, "image");
}