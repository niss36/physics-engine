mod circle;
mod line;
mod rectangle;

use crate::body::Body;
use crate::simulation::world::World;

pub fn render_world(world: &World) {
    for body in &world.bodies {
        match body {
            Body::Circle(circle) => circle::render_circle(circle),
            Body::Line(line) => line::render_line(line),
            Body::Rectangle(rectangle) => rectangle::render_rectangle(rectangle),
        }
    }
}
