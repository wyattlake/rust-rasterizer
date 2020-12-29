use crate::core::vector::*;
use crate::rendering::obj::*;
use image::{ImageBuffer, Rgb};
use std::cmp;

//Converts a point to barycentric coordinates
pub fn barycentric(x: f32, y: f32, points: &Vec<Vec3u>) -> Vec3f {
    let u = Vec3f(points[2].get(0) as f32 - points[0].get(0) as f32, points[1].get(0) as f32 - points[0].get(0) as f32, points[0].get(0) as f32 - x) * Vec3f(points[2].get(1) as f32 - points[0].get(1) as f32, points[1].get(1) as f32 - points[0].get(1) as f32, points[0].get(1) as f32 - y);
    
    //If the absolute value of the z coordinate of Vec3 is negative, then the triangle is degenerate
    if u.2.abs() < 1.0 {
        Vec3f(-1.0, 1.0, 1.0)
    }
    else {
        Vec3f(1.0 - (u.0 + u.1) / u.2, u.1 / u.2, u.0 / u.2)
    }
}

//Draws a triangle on a canvas given its vertices
pub fn draw_triangle(points: Vec<Vec3u>, zbuffer: &mut Vec<f32>, image: &mut ImageBuffer::<Rgb<u8>, Vec<u8>>, color: &[u8; 3]) {
    let image_width = image.width() as usize;
    let image_height = image.height() as usize;

    //Mutable min and max of the bounding box
    let mut bounding_box_min = Vec2u(image_width - 1, image_height as usize - 1);
    let mut bounding_box_max = Vec2u(0, 0);
    
    //Used to clamp the bounding box
    let clamp = Vec2u(image_width - 1, image_height - 1);

    //Finds the minimum and maximum points of the triangle
    for point in &points {
        for index in 0..2 {
            bounding_box_min.set(index, cmp::max(0, cmp::min(bounding_box_min.get(index), point.get(index))));
            bounding_box_max.set(index, cmp::min(clamp.get(index), cmp::max(bounding_box_max.get(index), point.get(index))));
        }
    }

    //Loops through points in bounding box
    for x in (bounding_box_min.0)..(bounding_box_max.0 + 1) {
        for y in (bounding_box_min.1)..(bounding_box_max.1 + 1) {
            let barycentric_point = barycentric(x as f32, y as f32, &points);
            //If the barycentric point is negative the point is outside of the triangle
            if barycentric_point.0 < 0.0 || barycentric_point.1 < 0.0 || barycentric_point.2 < 0.0 {
                continue;
            }
            //Finds the z value of the current point
            let mut z = 0.0;
            for index in 0..3 {
                z += points[index].get(2) as f32 * barycentric_point.get(index);
            }

            //Colors points in triangle if the z index is greater than the current z
            if zbuffer[x as usize + y as usize * image_width] < z {
                zbuffer[x as usize + y as usize * image_width] = z;
                image.get_pixel_mut(x as u32, y as u32).0 = color.clone();
            }
        }
    }
}

//Draws a triangle on a canvas given its vertices
pub fn draw_triangle_model(points: Vec<Vec3u>, shader: &Shader, model: &Model, zbuffer: &mut Vec<f32>, image: &mut ImageBuffer::<Rgb<u8>, Vec<u8>>) {
    let image_width = image.width() as usize;
    let image_height = image.height() as usize;

    //Mutable min and max of the bounding box
    let mut bounding_box_min = Vec2u(image_width - 1, image_height - 1);
    let mut bounding_box_max = Vec2u(0, 0);
    
    //Used to clamp the bounding box
    let clamp = Vec2u(image_width - 1, image_height - 1);

    //Finds the minimum and maximum points of the triangle
    for point in &points {
        for index in 0..2 {
            bounding_box_min.set(index, cmp::max(0, cmp::min(bounding_box_min.get(index), point.get(index))));
            bounding_box_max.set(index, cmp::min(clamp.get(index), cmp::max(bounding_box_max.get(index), point.get(index))));
        }
    }

    //Loops through points in bounding box
    for x in (bounding_box_min.0)..(bounding_box_max.0 + 1) {
        for y in (bounding_box_min.1)..(bounding_box_max.1 + 1) {
            let barycentric_point = barycentric(x as f32, y as f32, &points);
            //If the barycentric point is negative the point is outside of the triangle
            if barycentric_point.0 < 0.0 || barycentric_point.1 < 0.0 || barycentric_point.2 < 0.0 {
                continue;
            }
            //Finds the z value of the current point
            let mut z = 0.0;
            for index in 0..3 {
                z += points[index].get(2) as f32 * barycentric_point.get(index);
            }

            //Colors points in triangle if the z index is greater than the current z
            if zbuffer[x + y * image_width] < z {
                let color = shader.compute_color(&barycentric_point, model);
                zbuffer[x + y * image_width] = z;
                image.get_pixel_mut(x as u32, y as u32).0 = color;
            }
        }
    }
}
