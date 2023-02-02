use super::collisions::*;
use crate::body::*;
use crate::vec2::*;

use rand::{thread_rng, Rng};
use std::time::Duration;

#[derive(Debug)]
pub struct World {
    pub bodies: Vec<Body>,
    pub gravity: Vec2D,
}

impl World {
    pub fn new_populated(width: f64, height: f64, offset: f64, num_bodies: u32) -> Self {
        let top_border = Line::new(UNIT_DOWN, -offset);
        let right_border = Line::new(UNIT_LEFT, width - 1. - offset);
        let bottom_border = Line::new(UNIT_UP, height - 1. - offset);
        let left_border = Line::new(UNIT_RIGHT, -offset);

        let mut rng = thread_rng();

        let mut bodies: Vec<_> = [top_border, right_border, bottom_border, left_border]
            .into_iter()
            .map(|line| Body::Line(line))
            .collect();

        let circles = (0..num_bodies).map(|_| {
            let position_x = rng.gen_range(offset..=width - offset);
            let position_y = rng.gen_range(offset..=height - offset);

            let velocity_x = rng.gen::<f64>() - 0.5;
            let velocity_y = rng.gen::<f64>() - 0.5;

            let coefficient_of_restitution = rng.gen();

            let mass = rng.gen::<f64>() + 0.000001;

            Body::Circle(Circle {
                body: BaseBody {
                    position: Vec2D {
                        x: position_x,
                        y: position_y,
                    },
                    velocity: Vec2D {
                        x: velocity_x,
                        y: velocity_y,
                    },
                    coefficient_of_restitution,
                    inverse_mass: 1.0 / mass,
                },
                radius: 100. * mass,
            })
        });

        bodies.extend(circles);

        Self {
            bodies,
            gravity: Vec2D { x: 0., y: 10. },
        }
    }

    fn apply_gravity(&mut self, elapsed: &Duration) {
        for body in self.bodies.iter_mut() {
            let base_body = body.as_mut();
            if base_body.inverse_mass > 0. {
                base_body.velocity += &(&self.gravity * elapsed.as_secs_f64());
            }
        }
    }

    fn handle_collisions(&mut self, elapsed: &Duration) {
        let num_bodies = self.bodies.len();
        for i in 0..num_bodies {
            let (head, tail) = self.bodies.split_at_mut(i + 1);
            let this = &mut head[i];

            for that in tail {
                if let Some(contact) = generate_contact(this, that) {
                    let this_body = this.as_mut();
                    let that_body = that.as_mut();

                    let relative_velocity = &that_body.velocity - &this_body.velocity;
                    let relative_velocity_dot_normal =
                        relative_velocity.dot_product(&contact.normal);

                    let to_remove = relative_velocity_dot_normal
                        + 0.4
                        + (contact.distance + 1.) / elapsed.as_secs_f64();

                    if to_remove < 0. && contact.distance < 0. {
                        let impulse = &contact.normal
                            * (to_remove / (this_body.inverse_mass + that_body.inverse_mass));

                        this_body.velocity += &(&impulse * this_body.inverse_mass);
                        that_body.velocity -= &(&impulse * that_body.inverse_mass);
                    }
                }
            }
        }
    }

    fn integrate_bodies(&mut self, elapsed: &Duration) {
        for body in self.bodies.iter_mut() {
            body.as_mut().integrate(elapsed);
        }
    }

    pub fn tick(&mut self, elapsed: Duration) {
        self.apply_gravity(&elapsed);
        self.handle_collisions(&elapsed);
        self.integrate_bodies(&elapsed);
    }
}
