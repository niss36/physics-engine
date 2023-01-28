use crate::vec2::Vec2D;
use std::time::Duration;

#[derive(Debug)]
pub struct BaseBody {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub coefficient_of_restitution: f64,
    pub inverse_mass: f64,
}

impl BaseBody {
    pub fn integrate(&mut self, elapsed: &Duration) {
        self.position += &(&self.velocity * elapsed.as_secs_f64());
    }
}
