use crate::body::*;
use crate::vec2::*;

#[derive(Debug)]
pub struct Contact {
    pub normal: Vec2D,
    pub distance: f64,
}

impl Contact {
    fn flip(mut self) -> Self {
        self.normal = -self.normal;

        self
    }
}

pub fn generate_contact(this: &Body, that: &Body) -> Option<Contact> {
    use Body::*;

    match (this, that) {
        (Circle(this), Circle(that)) => Some(contacts::circle_circle(this, that)),
        (Circle(this), Line(that)) => Some(contacts::circle_line(this, that)),
        (Line(this), Circle(that)) => Some(contacts::circle_line(that, this).flip()),
        (Line(_), Line(_)) => None,
    }
}

mod contacts {
    use super::*;

    pub fn circle_circle(this: &Circle, that: &Circle) -> Contact {
        let normal = &that.body.position - &this.body.position;
        let length = normal.length();

        let distance = normal.length() - (this.radius + that.radius);

        Contact {
            normal: &normal / length,
            distance,
        }
    }

    pub fn circle_line(this: &Circle, that: &Line) -> Contact {
        let distance =
            that.normal.dot_product(&this.body.position) + that.origin_distance - this.radius;

        Contact {
            normal: -that.normal,
            distance,
        }
    }
}
