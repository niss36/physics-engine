use crate::{bounding_volume::BoundingVolume, vec2::Vec2D};

use super::BaseDynamicBody;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub body: BaseDynamicBody,
    pub half_width: f64,
    pub half_height: f64,
}

impl Rectangle {
    #[inline]
    pub fn to_bounding_volume(&self) -> BoundingVolume {
        let extents = Vec2D {
            x: self.half_width,
            y: self.half_height,
        };

        BoundingVolume {
            top_left: &self.body.position - &extents,
            bottom_right: &self.body.position + &extents,
        }
    }
}

impl AsRef<BaseDynamicBody> for Rectangle {
    fn as_ref(&self) -> &BaseDynamicBody {
        &self.body
    }
}

impl AsMut<BaseDynamicBody> for Rectangle {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        &mut self.body
    }
}
