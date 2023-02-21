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
        (Rectangle(_), Rectangle(_)) => true, // todo
        (Circle(_), Rectangle(_)) => true,    // todo
        (Rectangle(_), Circle(_)) => true,    // todo
        (Circle(_), Line(_)) => true,
        (Line(_), Circle(_)) => true,
        (Rectangle(_), Line(_)) => true,
        (Line(_), Rectangle(_)) => true,
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
        (Rectangle(this), Rectangle(that)) => None, // todo
        (Circle(this), Rectangle(that)) => None,    // todo
        (Rectangle(this), Circle(that)) => None,    // todo
        (Circle(this), Line(that)) => Some(contacts::line_circle(that, this).flip()),
        (Line(this), Circle(that)) => Some(contacts::line_circle(this, that)),
        (Rectangle(this), Line(that)) => Some(contacts::line_rectangle(that, this).flip()),
        (Line(this), Rectangle(that)) => Some(contacts::line_rectangle(this, that)),
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

    pub fn line_rectangle(this: &Line, that: &Rectangle) -> Contact {
        let offsets = [
            Vec2D {
                x: that.half_width,
                y: that.half_height,
            },
            Vec2D {
                x: that.half_width,
                y: -that.half_height,
            },
            Vec2D {
                x: -that.half_width,
                y: -that.half_height,
            },
            Vec2D {
                x: -that.half_width,
                y: that.half_height,
            },
        ];

        let distances = offsets.into_iter().map(|offset| {
            let point = &that.body.position + &offset;
            this.normal.dot_product(&point) + this.origin_distance
        });

        // Safe because there are always 4 elements
        let distance = distances.reduce(f64::min).unwrap();

        Contact {
            normal: this.normal,
            distance,
        }
    }
}
