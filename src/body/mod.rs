pub mod base;
pub mod circle;
pub mod line;
pub mod rectangle;

pub use base::*;
pub use circle::*;
pub use line::*;
pub use rectangle::*;

#[derive(Debug, Clone, Copy)]
pub enum Body {
    Circle(Circle),
    Line(Line),
    Rectangle(Rectangle),
}

impl AsRef<BaseBody> for Body {
    fn as_ref(&self) -> &BaseBody {
        match self {
            Self::Circle(circle) => circle.as_ref(),
            Self::Line(line) => line.as_ref(),
            Self::Rectangle(rectangle) => rectangle.as_ref(),
        }
    }
}

impl AsMut<BaseBody> for Body {
    fn as_mut(&mut self) -> &mut BaseBody {
        match self {
            Self::Circle(circle) => circle.as_mut(),
            Self::Line(line) => line.as_mut(),
            Self::Rectangle(rectangle) => rectangle.as_mut(),
        }
    }
}
