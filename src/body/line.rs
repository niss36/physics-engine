use super::BaseDynamicBody;
use crate::vec2::*;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub body: BaseDynamicBody,
    pub normal: Vec2D,
    pub origin_distance: f64,
}

impl Line {
    pub fn new(normal: Vec2D, origin_distance: f64) -> Self {
        Self {
            body: BaseDynamicBody {
                position: &normal * (-origin_distance / normal.length()),
                velocity: ZERO,
                coefficient_of_restitution: 1.,
                inverse_mass: 0.,
            },
            normal,
            origin_distance,
        }
    }
}

impl AsRef<BaseDynamicBody> for Line {
    fn as_ref(&self) -> &BaseDynamicBody {
        &self.body
    }
}

impl AsMut<BaseDynamicBody> for Line {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        &mut self.body
    }
}
