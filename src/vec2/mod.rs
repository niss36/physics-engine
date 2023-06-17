mod arithmetic;

pub use arithmetic::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

pub const ZERO: Vec2D = Vec2D { x: 0., y: 0. };
pub const UNIT_UP: Vec2D = Vec2D { x: 0., y: -1. };
pub const UNIT_RIGHT: Vec2D = Vec2D { x: 1., y: 0. };
pub const UNIT_DOWN: Vec2D = Vec2D { x: 0., y: 1. };
pub const UNIT_LEFT: Vec2D = Vec2D { x: -1., y: 0. };

impl Vec2D {
    pub fn dot_product(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn abs(&self) -> Self {
        Vec2D {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn clamp(&self, min: &Vec2D, max: &Vec2D) -> Vec2D {
        Vec2D {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }
}
