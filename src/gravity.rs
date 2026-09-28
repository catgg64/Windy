use std::{hash::Hash, println, time::Duration};

use crate::{CollisionObject, Coordinate2D, Object, WindyContext, twodimensional::shape::Line2D};

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

pub fn calculate_physics_step<T: Object>(windy_context: &WindyContext, objects: &mut Vec<T>, pull_angle: f64, dt: Duration) {
    for object in objects {
        if !object.is_colliding() {
            object.set_force(object.get_force() + object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64());
            object.set_direction((object.get_direction() + pull_angle) / 2.0);
            //println!("{object.get_mass() / windy_context.weight_dividor * pull_angle.cos() * dt.as_secs_f64()}")
            object.set_position(object.get_position().clone() - Coordinate2D{ x: object.get_force() * object.get_direction().cos() * dt.as_secs_f64(), y: object.get_force() * object.get_direction().sin() * dt.as_secs_f64() });
        }
    }
}
