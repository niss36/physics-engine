mod line;

pub use line::*;

#[derive(Debug, Clone, Copy)]
pub enum StaticBody {
    Line(Line),
}
