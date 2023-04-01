mod generation;

use super::collisions::*;
use crate::body::*;
use crate::vec2::*;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct World {
    pub bodies: Vec<Body>,
    pub gravity: Vec2D,
}

impl World {
    fn apply_gravity(&mut self, elapsed: &Duration) {
        let gravity = &self.gravity * elapsed.as_secs_f64();

        let bodies_with_mass = self
            .bodies
            .iter_mut()
            .map(Body::as_mut)
            .filter(|body| body.inverse_mass > 0.);

        for body in bodies_with_mass {
            body.velocity += &gravity;
        }
    }

    fn handle_collisions(&mut self) {
        let num_bodies = self.bodies.len();
        for i in 0..num_bodies {
            let (head, tail) = self.bodies.split_at_mut(i + 1);
            let this = &mut head[i];

            for that in tail {
                handle_collision(this, that);
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
        self.handle_collisions();
        self.integrate_bodies(&elapsed);
    }
}

fn handle_collision(this: &mut Body, that: &mut Body) {
    if !fast_collision_check(this, that) {
        return;
    }

    let Some(contact) = generate_contact(this, that) else { return };

    if contact.distance >= 0. {
        return;
    }

    let this_body = this.as_mut();
    let that_body = that.as_mut();

    apply_impulse(this_body, that_body, &contact);

    apply_correction(this_body, that_body, &contact);
}

fn apply_impulse(this_body: &mut BaseBody, that_body: &mut BaseBody, contact: &Contact) {
    let relative_velocity = &that_body.velocity - &this_body.velocity;
    let relative_velocity_dot_normal = relative_velocity.dot_product(&contact.normal);

    if relative_velocity_dot_normal > 0. {
        return;
    }

    let coefficient_of_restitution = f64::min(
        this_body.coefficient_of_restitution,
        that_body.coefficient_of_restitution,
    );

    let impulse_amount = (1. + coefficient_of_restitution) * relative_velocity_dot_normal
        / (this_body.inverse_mass + that_body.inverse_mass);

    let impulse = &contact.normal * impulse_amount;

    this_body.velocity += &(&impulse * this_body.inverse_mass);
    that_body.velocity -= &(&impulse * that_body.inverse_mass);
}

const CORRECTION_THRESHOLD: f64 = 0.05;
const CORRECTION_PERCENTAGE: f64 = 0.4;

fn apply_correction(this_body: &mut BaseBody, that_body: &mut BaseBody, contact: &Contact) {
    let correction_amount = (contact.distance + CORRECTION_THRESHOLD).min(0.)
        * CORRECTION_PERCENTAGE
        / (this_body.inverse_mass + that_body.inverse_mass);

    let correction = &contact.normal * correction_amount;

    this_body.position += &(&correction * this_body.inverse_mass);
    that_body.position -= &(&correction * that_body.inverse_mass);
}
