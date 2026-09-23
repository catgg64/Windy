use std::{hash::Hash};

use crate::{CollisionHash, CollisionObject, Coordinate2D, Object, WindyContext, twodimensional::shape::Line2D};

pub fn calculate_gravitational_pull<T: Object>(windy_context: WindyContext, objects: Vec<T>, position: &Coordinate2D) -> (f64, f64) {
    let mut angle = 0.0;
    let mut strengh = 0.0;
    for object in &objects {
        let object_position = object.get_position();
        angle += Line2D(position.clone(), object_position.clone()).angle();
        strengh += object.get_mass();
    }
    angle /= objects.len() as f64;
 
    (angle, strengh / windy_context.weight_dividor)
}

pub fn calculate_physics_step<T: Object + CollisionObject + Eq + Hash>(windy_context: WindyContext, mut objects: Vec<T>, collision_hash: CollisionHash<T>, pull_angle: f64) {
    for object in &mut objects {
        if collision_hash.contains(object) {
            object.set_position(object.get_position().clone() - Coordinate2D{ x: object.get_mass() / windy_context.weight_dividor * pull_angle.cos(), y: object.get_mass() / windy_context.weight_dividor * pull_angle.cos() });
        }
    }
}
