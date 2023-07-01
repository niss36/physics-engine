use super::BaseDynamicBody;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub body: BaseDynamicBody,
    pub radius: f64,
}

impl AsRef<BaseDynamicBody> for Circle {
    fn as_ref(&self) -> &BaseDynamicBody {
        &self.body
    }
}

impl AsMut<BaseDynamicBody> for Circle {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        &mut self.body
    }
}
