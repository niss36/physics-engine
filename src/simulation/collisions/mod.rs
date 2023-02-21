use crate::body::*;
use crate::vec2::*;

pub fn fast_collision_check(this: &Body, that: &Body) -> bool {
    use Body::*;

    match (this, that) {
        (Circle(this), Circle(that)) => {
            let this_to_that = &that.body.position - &this.body.position;

            let square_distance = this_to_that.length_squared();
            let square_total_radius = (this.radius + that.radius).powi(2);

            return square_distance < square_total_radius;
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
        let this_to_that = &that.body.position - &this.body.position;
        let length = this_to_that.length();

        let distance = length - (this.radius + that.radius);

        Contact {
            normal: &this_to_that / length,
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
