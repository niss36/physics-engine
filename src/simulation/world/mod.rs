mod generation;

use super::collisions::*;
use crate::body::*;
use crate::bounding_volume::BoundingVolume;
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

    fn detect_dynamic_collisions(&self) -> Vec<(Contact, usize, usize)> {
        let bounding_volumes: Vec<_> = self
            .dynamic_bodies
            .iter()
            .enumerate()
            .map(|(index, body)| (index, body.to_bounding_volume()))
            .collect();

        let Some(bvh) = BoundingVolumeHierarchyTree::new(bounding_volumes.as_slice()) else {
            return vec![];
        };

        let mut contacts: Vec<(Contact, usize, usize)> = vec![];

        for (i, this) in self.dynamic_bodies.iter().enumerate() {
            for j in bvh.get_overlapping_bodies(&bounding_volumes[i].1) {
                if j <= i {
                    continue;
                }

                let that = &self.dynamic_bodies[j];

                let Some(contact) = generate_contact_dynamic(this, that) else { continue; };

                if contact.distance >= 0. {
                    continue;
                }

                contacts.push((contact, i, j));
            }
        }

        contacts
    }

    fn handle_collisions(&mut self) {
        for this in &self.static_bodies {
            for that in &mut self.dynamic_bodies {
                handle_collision_static(this, that);
            }
        }

        let contacts = self.detect_dynamic_collisions();

        for _ in 0..10 {
            for (contact, i, j) in &contacts {
                if let Some(impulse) = get_impulse(
                    contact,
                    self.dynamic_bodies[*i].as_ref(),
                    self.dynamic_bodies[*j].as_ref(),
                ) {
                    let this_body = self.dynamic_bodies[*i].as_mut();
                    this_body.velocity += &(&impulse * this_body.inverse_mass);

                    let that_body = self.dynamic_bodies[*j].as_mut();
                    that_body.velocity -= &(&impulse * that_body.inverse_mass);
                }
            }
        }

        for (contact, i, j) in &contacts {
            let correction = get_correction(
                contact,
                self.dynamic_bodies[*i].as_ref(),
                self.dynamic_bodies[*j].as_ref(),
            );

            let this_body = self.dynamic_bodies[*i].as_mut();
            this_body.position += &(&correction * this_body.inverse_mass);

            let that_body = self.dynamic_bodies[*j].as_mut();
            that_body.position -= &(&correction * that_body.inverse_mass);
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
    let contact = generate_contact_static(this, that);

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

#[derive(Debug, Clone, PartialEq)]
enum BoundingVolumeHierarchyTree {
    Leaf(BoundingVolume, usize),
    Node(BoundingVolume, Box<Self>, Box<Self>),
}

impl BoundingVolumeHierarchyTree {
    fn new(bounding_volumes: &[(usize, BoundingVolume)]) -> Option<Self> {
        match bounding_volumes {
            [] => None,
            [(index, bounding_volume)] => {
                Some(BoundingVolumeHierarchyTree::Leaf(*bounding_volume, *index))
            }
            [(_, bounding_volume), remaining_bodies @ ..] => {
                let overall_bounding_volume = remaining_bodies
                    .iter()
                    .fold(*bounding_volume, |accumulator, (_, bounding_volume)| {
                        accumulator.union(bounding_volume)
                    });

                let BoundingVolume {
                    top_left: Vec2D { x: min_x, y: min_y },
                    bottom_right: Vec2D { x: max_x, y: max_y },
                } = overall_bounding_volume;

                let is_main_axis_x = (max_x - min_x) > (max_y - min_y);

                let overall_mid_point = if is_main_axis_x {
                    (min_x + max_x) / 2.
                } else {
                    (min_y + max_y) / 2.
                };

                let mut left_bounding_volumes = vec![];
                let mut right_bounding_volumes = vec![];

                for &(index, bounding_volume) in bounding_volumes {
                    let volume_mid_point = if is_main_axis_x {
                        (bounding_volume.top_left.x + bounding_volume.bottom_right.x) / 2.
                    } else {
                        (bounding_volume.top_left.y + bounding_volume.bottom_right.y) / 2.
                    };

                    if volume_mid_point < overall_mid_point {
                        left_bounding_volumes.push((index, bounding_volume));
                    } else {
                        right_bounding_volumes.push((index, bounding_volume));
                    }
                }

                let threshold = 1.max(bounding_volumes.len() / 16);

                while left_bounding_volumes.len() < threshold {
                    left_bounding_volumes.push(right_bounding_volumes.pop()?);
                }

                while right_bounding_volumes.len() < threshold {
                    right_bounding_volumes.push(left_bounding_volumes.pop()?);
                }

                Some(Self::Node(
                    overall_bounding_volume,
                    Box::new(Self::new(left_bounding_volumes.as_slice())?),
                    Box::new(Self::new(right_bounding_volumes.as_slice())?),
                ))
            }
        }
    }

    fn get_overlapping_bodies(&self, query_bounding_volume: &BoundingVolume) -> Vec<usize> {
        let mut result = vec![];

        fn aux(
            current_node: &BoundingVolumeHierarchyTree,
            query_bounding_volume: &BoundingVolume,
            result: &mut Vec<usize>,
        ) {
            match current_node {
                BoundingVolumeHierarchyTree::Leaf(bounding_volume, index) => {
                    if bounding_volume.is_intersecting(query_bounding_volume) {
                        result.push(*index);
                    }
                }
                BoundingVolumeHierarchyTree::Node(bounding_volume, left_child, right_child) => {
                    if bounding_volume.is_intersecting(query_bounding_volume) {
                        aux(left_child, query_bounding_volume, result);
                        aux(right_child, query_bounding_volume, result);
                    }
                }
            }
        }

        aux(self, query_bounding_volume, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_square(top_left: Vec2D, width: f64) -> BoundingVolume {
        BoundingVolume {
            top_left,
            bottom_right: Vec2D {
                x: top_left.x + width,
                y: top_left.y + width,
            },
        }
    }

    #[test]
    fn test_bvh_new() {
        let bv1 = create_square(ZERO, 10.);
        let bv2 = create_square(Vec2D { x: 20., y: 0. }, 10.);
        let bv3 = create_square(Vec2D { x: 20., y: 20. }, 10.);
        let bv4 = create_square(Vec2D { x: 0., y: 20. }, 10.);

        let bounding_volumes: Vec<_> = vec![bv1, bv2, bv3, bv4].into_iter().enumerate().collect();

        let bvh = BoundingVolumeHierarchyTree::new(&bounding_volumes);

        use BoundingVolumeHierarchyTree::{Leaf, Node};

        assert_eq!(
            bvh,
            Some(Node(
                bv1.union(&bv2).union(&bv3).union(&bv4),
                Box::new(Node(
                    bv1.union(&bv2),
                    Box::new(Leaf(bv1, 0)),
                    Box::new(Leaf(bv2, 1))
                )),
                Box::new(Node(
                    bv3.union(&bv4),
                    Box::new(Leaf(bv4, 3)),
                    Box::new(Leaf(bv3, 2)),
                ))
            ))
        );
    }
}
