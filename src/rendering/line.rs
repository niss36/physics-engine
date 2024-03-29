use macroquad::prelude::*;

pub fn render_line(line: &crate::body::Line) {
    let a = line.normal.x;
    let b = line.normal.y;
    let c = line.origin_distance;

    if b != 0. {
        let a0 = (-a / b) as f32;
        let b0 = (-c / b) as f32;

        let x1 = 0.;
        let x2 = screen_width();

        let y1 = a0 * x1 + b0;
        let y2 = a0 * x2 + b0;

        draw_line(x1, y1, x2, y2, 1., BLACK);
    } else {
        let c0 = (-c / a) as f32;

        let y1 = 0.;
        let y2 = screen_height();

        draw_line(c0, y1, c0, y2, 1., BLACK);
    }
}
