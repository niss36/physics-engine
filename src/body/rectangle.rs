use super::BaseDynamicBody;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub body: BaseDynamicBody,
    pub half_width: f64,
    pub half_height: f64,
}

impl Rectangle {
    pub fn get_min_max(&self) -> (f64, f64, f64, f64) {
        let pos = self.body.position;

        (
            pos.x - self.half_width,
            pos.x + self.half_width,
            pos.y - self.half_height,
            pos.y + self.half_height,
        )
    }
}

impl AsRef<BaseDynamicBody> for Rectangle {
    fn as_ref(&self) -> &BaseDynamicBody {
        &self.body
    }
}

impl AsMut<BaseDynamicBody> for Rectangle {
    fn as_mut(&mut self) -> &mut BaseDynamicBody {
        &mut self.body
    }
}
