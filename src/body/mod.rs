pub mod base;
pub mod circle;
pub mod line;
pub mod rectangle;

pub use base::*;
pub use circle::*;
pub use line::*;
pub use rectangle::*;

#[derive(Debug, Clone, Copy)]
pub enum DynamicBody {
    Circle(Circle),
    Line(Line),
    Rectangle(Rectangle),
}

impl AsRef<BaseDynamicBody> for DynamicBody {
    fn as_ref(&self) -> &BaseDynamicBody {
        match self {
            Self::Circle(circle) => circle.as_ref(),
            Self::Line(line) => line.as_ref(),
            Self::Rectangle(rectangle) => rectangle.as_ref(),
        }
    }
}

impl AsMut<BaseDynamicBody> for DynamicBody {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        match self {
            Self::Circle(circle) => circle.as_mut(),
            Self::Line(line) => line.as_mut(),
            Self::Rectangle(rectangle) => rectangle.as_mut(),
        }
    }
}
