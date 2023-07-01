use crate::{bounding_volume::BoundingVolume, vec2::Vec2D};

use super::BaseDynamicBody;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub body: BaseDynamicBody,
    pub radius: f64,
}

impl Circle {
    #[inline]
    pub fn to_bounding_volume(&self) -> BoundingVolume {
        let extents = Vec2D {
            x: self.radius,
            y: self.radius,
        };

        BoundingVolume {
            top_left: &self.body.position - &extents,
            bottom_right: &self.body.position + &extents,
        }
    }
}

impl AsRef<BaseDynamicBody> for Circle {
    fn as_ref(&self) -> &BaseDynamicBody {
        &self.body
    }
}

impl AsMut<BaseDynamicBody> for Circle {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        &mut self.body
    }
}
