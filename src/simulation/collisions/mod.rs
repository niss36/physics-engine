use crate::body::*;
use crate::vec2::*;

pub fn fast_collision_check(this: &Body, that: &Body) -> bool {
    use Body::*;

    match (this, that) {
        (Circle(this), Circle(that)) => {
            let normal = &that.body.position - &this.body.position;

            return normal.dot_product(&normal)
                < (this.radius + that.radius) * (this.radius + that.radius);
        }
        (Circle(_), Line(_)) => true,
        (Line(_), Circle(_)) => true,
        (Line(_), Line(_)) => false,
    }
}

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
        (Circle(this), Line(that)) => Some(contacts::line_circle(that, this).flip()),
        (Line(this), Circle(that)) => Some(contacts::line_circle(this, that)),
        (Line(_), Line(_)) => None,
    }
}

mod contacts {
    use super::*;

    pub fn circle_circle(this: &Circle, that: &Circle) -> Contact {
        let normal = &that.body.position - &this.body.position;
        let length = normal.length();

        let distance = length - (this.radius + that.radius);

        Contact {
            normal: &normal / length,
            distance,
        }
    }

    pub fn line_circle(this: &Line, that: &Circle) -> Contact {
        let distance =
            this.normal.dot_product(&that.body.position) + this.origin_distance - that.radius;

        Contact {
            normal: this.normal,
            distance,
        }
    }
}
