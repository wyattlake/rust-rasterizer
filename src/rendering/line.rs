use image::{ImageBuffer, Rgb};
use std::mem;

//Draws a line on a canvas
pub fn draw_line(x0_input: i32, y0_input: i32, x1_input: i32, y1_input: i32, canvas: &mut ImageBuffer::<Rgb<u8>, Vec<u8>>, color: &[u8; 3]) {
    let mut steep = false;
    let (mut x0, mut x1, mut y0, mut y1): (i32, i32, i32, i32);

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
        mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let mut error2 = 0;
    let derror2 = dy.abs() * 2;
    let mut y = y0;

    //Determines coordinates and sets the color on the canvas
    for x in x0..(x1 + 1) {
        if steep {
            canvas.put_pixel(y as u32, x as u32, Rgb(color.clone()));
        }
        else {
            canvas.put_pixel(x as u32, y as u32, Rgb(color.clone()));
        }

        error2 += derror2;
        if error2 > dx {
            if y1 > y0 {
                y += 1;
            }
            else {
                y -= 1;
            }
            error2 -= dx * 2;
        }
    }
}