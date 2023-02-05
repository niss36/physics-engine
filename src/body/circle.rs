use super::BaseBody;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub body: BaseBody,
    pub radius: f64,
}

impl AsRef<BaseBody> for Circle {
    fn as_ref(&self) -> &BaseBody {
        &self.body
    }
}

impl AsMut<BaseBody> for Circle {
    fn as_mut(&mut self) -> &mut BaseBody {
        &mut self.body
    }
}
