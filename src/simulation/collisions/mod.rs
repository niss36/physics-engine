use crate::body::*;
use crate::vec2::*;

#[derive(Debug, Clone, Copy)]
pub struct Contact {
    pub normal: Vec2D,
    pub distance: f64,
}

impl Contact {
    pub fn flip(mut self) -> Self {
        self.normal = -self.normal;

        self
    }
}

pub fn generate_contact_static(this: &StaticBody, that: &DynamicBody) -> Contact {
    match (this, that) {
        (StaticBody::Line(this), DynamicBody::Circle(that)) => contacts::line_circle(this, that),
        (StaticBody::Line(this), DynamicBody::Rectangle(that)) => {
            contacts::line_rectangle(this, that)
        }
    }
}

pub fn generate_contact_dynamic(this: &DynamicBody, that: &DynamicBody) -> Option<Contact> {
    use DynamicBody::*;

    match (this, that) {
        (Circle(this), Circle(that)) => Some(contacts::circle_circle(this, that)),
        (Rectangle(this), Rectangle(that)) => contacts::rectangle_rectangle(this, that),
        (Circle(this), Rectangle(that)) => Some(contacts::circle_rectangle(this, that)),
        (Rectangle(this), Circle(that)) => Some(contacts::circle_rectangle(that, this).flip()),
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

    pub fn rectangle_rectangle(this: &Rectangle, that: &Rectangle) -> Option<Contact> {
        let displacement = &that.body.position - &this.body.position;

        let x_overlap = this.half_width + that.half_width - displacement.x.abs();
        let y_overlap = this.half_height + that.half_height - displacement.y.abs();

        if x_overlap <= 0. || y_overlap <= 0. {
            return None;
        }

        if x_overlap < y_overlap {
            let normal = if displacement.x < 0. {
                UNIT_LEFT
            } else {
                UNIT_RIGHT
            };

            Some(Contact {
                normal,
                distance: -x_overlap,
            })
        } else {
            let normal = if displacement.y < 0. {
                UNIT_UP
            } else {
                UNIT_DOWN
            };

            Some(Contact {
                normal,
                distance: -y_overlap,
            })
        }
    }

    pub fn circle_rectangle(this: &Circle, that: &Rectangle) -> Contact {
        let displacement = &that.body.position - &this.body.position;

        let clamped_displacement = displacement.clamp(
            &Vec2D {
                x: -that.half_width,
                y: -that.half_height,
            },
            &Vec2D {
                x: that.half_width,
                y: that.half_height,
            },
        );

        let is_inside = clamped_displacement == displacement;

        let closest_point = if is_inside {
            if displacement.x.abs() > displacement.y.abs() {
                Vec2D {
                    x: clamped_displacement.x.signum() * that.half_width,
                    y: clamped_displacement.y,
                }
            } else {
                Vec2D {
                    x: clamped_displacement.x,
                    y: clamped_displacement.y.signum() * that.half_height,
                }
            }
        } else {
            clamped_displacement
        };

        let normal = &displacement - &closest_point;
        let length = normal.length();

        let distance = length - this.radius;

        if is_inside {
            Contact {
                normal: -(&normal / length),
                distance,
            }
        } else {
            Contact {
                normal: &normal / length,
                distance,
            }
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
