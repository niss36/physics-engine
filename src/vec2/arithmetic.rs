use super::Vec2D;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<'b> Add<&'b Vec2D> for &Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: &'b Vec2D) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: &Vec2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'b> Sub<&'b Vec2D> for &Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: &'b Vec2D) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<&Vec2D> for Vec2D {
    fn sub_assign(&mut self, rhs: &Vec2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for &Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Vec2D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for &Vec2D {
    type Output = Vec2D;

    fn div(self, rhs: f64) -> Self::Output {
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Vec2D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2D {
    type Output = Vec2D;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;

        self
    }
}

impl Neg for &Vec2D {
    type Output = Vec2D;

    fn neg(self) -> Self::Output {
        Vec2D {
            x: -self.x,
            y: -self.y,
        }
    }
}
