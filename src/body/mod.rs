pub mod base;
pub mod circle;
pub mod line;

pub use base::*;
pub use circle::*;
pub use line::*;

#[derive(Debug)]
pub enum Body {
    Circle(Circle),
    Line(Line),
}

impl AsRef<BaseBody> for Body {
    fn as_ref(&self) -> &BaseBody {
        match self {
            Self::Circle(circle) => circle.as_ref(),
            Self::Line(line) => line.as_ref(),
        }
    }
}

impl AsMut<BaseBody> for Body {
    fn as_mut(&mut self) -> &mut BaseBody {
        match self {
            Self::Circle(circle) => circle.as_mut(),
            Self::Line(line) => line.as_mut(),
        }
    }
}
