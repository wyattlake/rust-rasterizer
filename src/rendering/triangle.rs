use crate::core::vector::*;
use crate::core::color::Color;
use crate::core::canvas::Canvas;
use crate::misc::utils::*;

//Converts a point to barycentric coordinates
pub fn barycentric(x: f32, y: f32, points: &Vec<Vec2>) -> Vec3 {
    let u = Vec3(points[2].get(0) - points[0].get(0), points[1].get(0) - points[0].get(0), points[0].get(0) - x) * Vec3(points[2].get(1) - points[0].get(1), points[1].get(1) - points[0].get(1), points[0].get(1) - y);
    
    //If the absolute value of the z coordinate of Vec3 is negative, then the triangle is degenerate
    if u.2.abs() < 1.0 {
        Vec3(-1.0, 1.0, 1.0)
    }
    else {
        Vec3(1.0 - (u.0 + u.1) / u.2, u.1 / u.2, u.0 / u.2)
    }
}

//Draws a triangle on a canvas given its vertices
pub fn draw_triangle(points: Vec<Vec2>, canvas: &mut Canvas, color: &Color) {
    //Mutable min and max of the bounding box
    let mut bounding_box_min = Vec2(canvas.width as f32 - 1.0, canvas.height as f32 - 1.0);
    let mut bounding_box_max = Vec2(0.0, 0.0);
    
    //Used to clamp the bounding box
    let clamp = Vec2(canvas.width as f32 - 1.0, canvas.height as f32 - 1.0);

    //Finds the minimum and maximum points of the triangle
    for point in &points {
        for index in 0..2 {
            bounding_box_min.set(index, max_float(0.0, min_float(bounding_box_min.get(index), point.get(index))));
            bounding_box_max.set(index, min_float(clamp.get(index), max_float(bounding_box_max.get(index), point.get(index))));
        }
    }

    //Loops through points in bounding box
    for x in (bounding_box_min.0 as i32)..(bounding_box_max.0 + 1.0) as i32 {
        for y in (bounding_box_min.1) as i32..(bounding_box_max.1 + 1.0) as i32 {
            let barycentric_point = barycentric(x as f32, y as f32, &points);
            //If the barycentric point is negative the point is outside of the triangle
            if barycentric_point.0 < 0.0 || barycentric_point.1 < 0.0 || barycentric_point.2 < 0.0 {
                continue;
            }
            //Colors points in triangle
            canvas.set(color.clone(), x, y);
        }
    }
}
