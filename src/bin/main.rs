use rust_rasterizer::rendering::obj::*;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Rgb, DynamicImage};
use std::time::Instant;
use std::fs::File;

//Width and height of the scene
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    //Image where color is stored
    let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(WIDTH, HEIGHT);

    let file = File::open("src/models/model.obj").unwrap();
    let mut model = Model::new(file);
    let img = ImageReader::open("src/models/texture.tga").unwrap().decode().unwrap();
    model.load_texture(img);

    println!("Render started...");
    let now = Instant::now();

    render_model(&model, &mut image);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    DynamicImage::ImageRgb8(image).flipv().save("result.png").unwrap();
}