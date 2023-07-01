use crate::vec2::Vec2D;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub normal: Vec2D,
    pub origin_distance: f64,
}
