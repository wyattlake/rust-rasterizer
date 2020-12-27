#[cfg(test)]

mod tests {
    use rust_rasterizer::core::canvas::Canvas;
    use rust_rasterizer::core::color::Color;
    use rust_rasterizer::rendering::line::*;

    #[test]
    //Tests drawing lines
    fn line_drawing_tests() {
        let mut canvas = Canvas::new(10, 10); 
        draw_line(0, 0, 9, 0, &mut canvas, &Color(1.0, 1.0, 1.0));
        assert_eq!(canvas.get(0, 0).unwrap(), &Color(1.0, 1.0, 1.0));
        assert_eq!(canvas.get(9, 0).unwrap(), &Color(1.0, 1.0, 1.0));
    }
}