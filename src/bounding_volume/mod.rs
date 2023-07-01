use crate::vec2::Vec2D;

#[derive(Debug, Clone, Copy)]
pub struct BoundingVolume {
    pub top_left: Vec2D,
    pub bottom_right: Vec2D,
}

impl BoundingVolume {
    #[inline]
    pub fn is_intersecting(&self, other: &BoundingVolume) -> bool {
        if self.bottom_right.x < other.top_left.x || self.top_left.x > other.bottom_right.x {
            return false;
        }

        if self.bottom_right.y < other.top_left.y || self.top_left.y > other.bottom_right.y {
            return false;
        }

        true
    }
}
