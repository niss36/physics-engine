use crate::vec2::Vec2D;

#[derive(Debug, Clone, Copy)]
pub struct BaseDynamicBody {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub coefficient_of_restitution: f64,
    pub inverse_mass: f64,
}

impl BaseDynamicBody {
    pub fn integrate(&mut self, elapsed: f64) {
        self.position += &(&self.velocity * elapsed);
    }
}
