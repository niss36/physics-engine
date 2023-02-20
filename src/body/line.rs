use super::BaseBody;
use crate::vec2::*;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub body: BaseBody,
    pub normal: Vec2D,
    pub origin_distance: f64,
}

impl Line {
    pub fn new(normal: Vec2D, origin_distance: f64) -> Self {
        Self {
            body: BaseBody {
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

impl AsRef<BaseBody> for Line {
    fn as_ref(&self) -> &BaseBody {
        &self.body
    }
}

impl AsMut<BaseBody> for Line {
    fn as_mut(&mut self) -> &mut BaseBody {
        &mut self.body
    }
}
