use crate::vec2::Vec2D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingVolume {
    pub top_left: Vec2D,
    pub bottom_right: Vec2D,
}

impl BoundingVolume {
    #[inline]
    pub fn is_intersecting(&self, other: &BoundingVolume) -> bool {
        if self.bottom_right.x <= other.top_left.x || self.top_left.x >= other.bottom_right.x {
            return false;
        }

        if self.bottom_right.y <= other.top_left.y || self.top_left.y >= other.bottom_right.y {
            return false;
        }

        true
    }

    pub fn union(&self, other: &BoundingVolume) -> BoundingVolume {
        BoundingVolume {
            top_left: self.top_left.min(&other.top_left),
            bottom_right: self.bottom_right.max(&other.bottom_right),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec2::ZERO;

    use super::*;

    fn create_square(top_left: Vec2D, width: f64) -> BoundingVolume {
        BoundingVolume {
            top_left,
            bottom_right: Vec2D {
                x: top_left.x + width,
                y: top_left.y + width,
            },
        }
    }

    #[test]
    fn test_is_intersecting_returns_true_for_intersecting_bounding_volumes() {
        let bv1 = create_square(ZERO, 10.);
        let bv2 = create_square(Vec2D { x: 5., y: 5. }, 10.);

        assert!(bv1.is_intersecting(&bv2));
    }

    #[test]
    fn test_is_intersecting_returns_true_for_the_same_bounding_volume() {
        let bv = create_square(ZERO, 10.);

        assert!(bv.is_intersecting(&bv));
    }

    #[test]
    fn test_is_intersecting_returns_false_for_touching_bounding_volumes() {
        let bv1 = create_square(ZERO, 10.);
        let bv2 = create_square(Vec2D { x: 10., y: 0. }, 10.);

        assert!(!bv1.is_intersecting(&bv2));
    }

    #[test]
    fn test_is_intersecting_returns_false_for_non_intersecting_bounding_volumes() {
        let bv1 = create_square(ZERO, 10.);
        let bv2 = create_square(Vec2D { x: 20., y: 0. }, 10.);

        assert!(!bv1.is_intersecting(&bv2));
    }

    #[test]
    fn test_union_works() {
        let bv1 = create_square(ZERO, 10.);
        let bv2 = create_square(Vec2D { x: 5., y: 5. }, 10.);

        let expected_result = create_square(ZERO, 15.);

        assert_eq!(bv1.union(&bv2), expected_result);
        assert_eq!(bv2.union(&bv1), expected_result);
    }
}
