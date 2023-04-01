use macroquad::prelude::*;

pub fn render_circle(circle: &crate::body::Circle) {
    draw_poly(
        circle.body.position.x as f32,
        circle.body.position.y as f32,
        40,
        circle.radius as f32,
        0.,
        BLACK,
    );
}
