use crate::core::canvas::Canvas;
use crate::core::color::Color;
use std::mem;

//Draws a line on a canvas
pub fn draw_line(x0_input: i32, y0_input: i32, x1_input: i32, y1_input: i32, canvas: &mut Canvas, color: &Color) {
    let mut steep = false;
    let (mut x0, mut x1, y0, y1): (i32, i32, i32, i32);

    //Determines if the slope of the line is greater than 1
    if (x0_input - x1_input).abs() < (y0_input - y1_input).abs() {
        //Swapped values for steep lines
        x0 = y0_input;
        y0 = x0_input;
        x1 = y1_input;
        y1 = x1_input;
        steep = true;
    }
    else {
        x0 = x0_input;
        y0 = y0_input;
        x1 = x1_input;
        y1 = y1_input;
    }

    //Draws line from left to right
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
    }

    //Determines coordinates and sets the color on the canvas
    for x in x0..(x1 + 1) {
        let t = (x - x0) as f32 / (x1 - x0) as f32;
        let y = (y0 as f32 * (1.0 - t) + y1 as f32 * t).round() as i32;
        if steep {
            canvas.set(color.clone(), y, x);
        }
        else {
            canvas.set(color.clone(), x, y);
        }
    }
}