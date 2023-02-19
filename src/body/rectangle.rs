use super::BaseBody;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub body: BaseBody,
    pub half_width: f64,
    pub half_height: f64,
}

impl AsRef<BaseBody> for Rectangle {
    fn as_ref(&self) -> &BaseBody {
        &self.body
    }
}

impl AsMut<BaseBody> for Rectangle {
    fn as_mut(&mut self) -> &mut BaseBody {
        &mut self.body
    }
}
