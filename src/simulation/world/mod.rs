mod generation;

use super::collisions::*;
use crate::body::*;
use crate::vec2::*;

#[derive(Debug, Clone)]
pub struct World {
    pub static_bodies: Vec<StaticBody>,
    pub dynamic_bodies: Vec<DynamicBody>,
    pub gravity: Vec2D,
}

impl World {
    fn apply_gravity(&mut self, elapsed: f64) {
        let gravity = &self.gravity * elapsed;

        for body in &mut self.dynamic_bodies {
            body.as_mut().velocity += &gravity;
        }
    }

    fn handle_collisions(&mut self) {
        for this in &self.static_bodies {
            for that in &mut self.dynamic_bodies {
                handle_collision_static(this, that);
            }
        }

        let num_bodies = self.dynamic_bodies.len();
        for i in 0..num_bodies {
            let (head, tail) = self.dynamic_bodies.split_at_mut(i + 1);
            let this = &mut head[i];

            for that in tail {
                handle_collision_dynamic(this, that);
            }
        }
    }

    fn integrate_bodies(&mut self, elapsed: f64) {
        for body in self.dynamic_bodies.iter_mut() {
            body.as_mut().integrate(elapsed);
        }
    }

    pub fn tick(&mut self, elapsed: f64) {
        self.apply_gravity(elapsed);
        self.handle_collisions();
        self.integrate_bodies(elapsed);
    }
}

fn handle_collision_static(this: &StaticBody, that: &mut DynamicBody) {
    let Some(contact) = generate_contact_static(this, that) else { return };

    if contact.distance >= 0. {
        return;
    }

    let this_body = BaseDynamicBody {
        position: ZERO,
        velocity: ZERO,
        coefficient_of_restitution: 1.,
        inverse_mass: 0.,
    };
    let that_body = that.as_mut();

    if let Some(impulse) = get_impulse(&contact, &this_body, that_body) {
        that_body.velocity -= &(&impulse * that_body.inverse_mass);
    }

    let correction = get_correction(&contact, &this_body, that_body);
    that_body.position -= &(&correction * that_body.inverse_mass);
}

fn handle_collision_dynamic(this: &mut DynamicBody, that: &mut DynamicBody) {
    if !fast_collision_check(this, that) {
        return;
    }

    let Some(contact) = generate_contact_dynamic(this, that) else { return };

    if contact.distance >= 0. {
        return;
    }

    let this_body = this.as_mut();
    let that_body = that.as_mut();

    if let Some(impulse) = get_impulse(&contact, this_body, that_body) {
        this_body.velocity += &(&impulse * this_body.inverse_mass);
        that_body.velocity -= &(&impulse * that_body.inverse_mass);
    };

    let correction = get_correction(&contact, this_body, that_body);

    this_body.position += &(&correction * this_body.inverse_mass);
    that_body.position -= &(&correction * that_body.inverse_mass);
}

fn get_impulse(
    contact: &Contact,
    this_body: &BaseDynamicBody,
    that_body: &BaseDynamicBody,
) -> Option<Vec2D> {
    let relative_velocity = &that_body.velocity - &this_body.velocity;
    let relative_velocity_dot_normal = relative_velocity.dot_product(&contact.normal);

    if relative_velocity_dot_normal > 0. {
        return None;
    }

    let coefficient_of_restitution = f64::min(
        this_body.coefficient_of_restitution,
        that_body.coefficient_of_restitution,
    );

    let impulse_amount = (1. + coefficient_of_restitution) * relative_velocity_dot_normal
        / (this_body.inverse_mass + that_body.inverse_mass);

    Some(&contact.normal * impulse_amount)
}

const CORRECTION_THRESHOLD: f64 = 0.05;
const CORRECTION_PERCENTAGE: f64 = 0.4;

fn get_correction(
    contact: &Contact,
    this_body: &BaseDynamicBody,
    that_body: &BaseDynamicBody,
) -> Vec2D {
    let correction_amount = (contact.distance + CORRECTION_THRESHOLD).min(0.)
        * CORRECTION_PERCENTAGE
        / (this_body.inverse_mass + that_body.inverse_mass);

    &contact.normal * correction_amount
}
