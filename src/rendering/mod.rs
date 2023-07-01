mod circle;
mod line;
mod rectangle;

use crate::body::DynamicBody;
use crate::simulation::world::World;

pub fn render_world(world: &World) {
    for body in &world.dynamic_bodies {
        match body {
            DynamicBody::Circle(circle) => circle::render_circle(circle),
            DynamicBody::Line(line) => line::render_line(line),
            DynamicBody::Rectangle(rectangle) => rectangle::render_rectangle(rectangle),
        }
    }
}
