use macroquad::rand::gen_range;

use crate::{body::*, vec2::*};

use super::World;

fn random_position(width: f64, height: f64, offset: f64) -> Vec2D {
    Vec2D {
        x: gen_range(offset, width - offset),
        y: gen_range(offset, height - offset),
    }
}

const MAX_INITIAL_VELOCITY: f64 = 5.;

fn random_velocity() -> Vec2D {
    Vec2D {
        x: gen_range(-MAX_INITIAL_VELOCITY, MAX_INITIAL_VELOCITY),
        y: gen_range(-MAX_INITIAL_VELOCITY, MAX_INITIAL_VELOCITY),
    }
}

fn random_base_dynamic_body(width: f64, height: f64, offset: f64) -> BaseDynamicBody {
    let position = random_position(width, height, offset);

    let velocity = random_velocity();

    let coefficient_of_restitution = gen_range(0., 1.);

    let mass = gen_range(0., 1.) + 0.000001;

    BaseDynamicBody {
        position,
        velocity,
        coefficient_of_restitution,
        inverse_mass: 1. / mass,
    }
}

const SIZE_TO_MASS_RATIO: f64 = 10.;

fn random_circle(width: f64, height: f64, offset: f64) -> Circle {
    let body = random_base_dynamic_body(width, height, offset);

    Circle {
        body,
        radius: SIZE_TO_MASS_RATIO / body.inverse_mass,
    }
}

fn random_rectangle(width: f64, height: f64, offset: f64) -> Rectangle {
    let body = random_base_dynamic_body(width, height, offset);

    let aspect_ratio = gen_range(0.25, 0.75);

    let half_width = aspect_ratio * SIZE_TO_MASS_RATIO / body.inverse_mass;
    let half_height = (1. - aspect_ratio) * SIZE_TO_MASS_RATIO / body.inverse_mass;

    Rectangle {
        body,
        half_width,
        half_height,
    }
}

impl World {
    pub fn generate(width: f64, height: f64, offset: f64, num_bodies: u32) -> Self {
        let top_border = Line {
            normal: UNIT_DOWN,
            origin_distance: -offset,
        };
        let right_border = Line {
            normal: UNIT_LEFT,
            origin_distance: width - 1. - offset,
        };
        let bottom_border = Line {
            normal: UNIT_UP,
            origin_distance: height - 1. - offset,
        };
        let left_border = Line {
            normal: UNIT_RIGHT,
            origin_distance: -offset,
        };

        let static_bodies: Vec<_> = [top_border, right_border, bottom_border, left_border]
            .into_iter()
            .map(StaticBody::Line)
            .collect();

        let mut dynamic_bodies = vec![];

        let circles =
            (0..num_bodies).map(|_| DynamicBody::Circle(random_circle(width, height, offset)));

        dynamic_bodies.extend(circles);

        let rectangles = (0..num_bodies)
            .map(|_| DynamicBody::Rectangle(random_rectangle(width, height, offset)));

        dynamic_bodies.extend(rectangles);

        Self {
            static_bodies,
            dynamic_bodies,
            gravity: Vec2D { x: 0., y: 0. },
        }
    }
}
