use macroquad::prelude::*;

pub fn render_rectangle(rectangle: &crate::body::Rectangle) {
    let x = (rectangle.body.position.x - rectangle.half_width) as f32;
    let y = (rectangle.body.position.y - rectangle.half_height) as f32;

    let width = (rectangle.half_width * 2.) as f32;
    let height = (rectangle.half_height * 2.) as f32;

    draw_rectangle_lines(x, y, width, height, 1., BLACK);
}
