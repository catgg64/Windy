use std::{time::Duration, vec};

use crate::{Coordinate2D, Object, WindyContext, twodimensional::shape::Line2D};

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
    for object in objects.iter_mut() {
        let object_velocity = object.get_velocity();
        object.set_velocity((object_velocity.0 + (object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64()) * pull_angle.cos(), object_velocity.1 + (object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64()) * pull_angle.sin()));
    }
}

pub fn calculate_collision_pushback<T: Object>(windy_context: &WindyContext, objects: &mut Vec<T>, dt: Duration) {
    let dt = dt.as_secs_f64();

    for object in objects.iter_mut() {
        let mtv = match object.get_collision_info().first().and_then(|c| c.mtv) {
            Some(m) => m,
            None => continue,
        };

        let depth = (mtv.x * mtv.x + mtv.y * mtv.y).sqrt();
        if depth < 1e-12 { continue; }
        let (nx, ny) = (mtv.x / depth, mtv.y / depth);

        // Move out by exactly the real overlap (tiny extra so we end up just clear).
        let pos = object.get_position().clone();
        let push = depth + 1e-6;
        object.set_position(Coordinate2D { x: pos.x + nx * push, y: pos.y + ny * push });

        // Only the velocity into the surface changes.
        let mut v = object.get_velocity();
        let vn = v.0 * nx + v.1 * ny;
        if vn < 0.0 {
            let rest_speed = object.get_mass() / windy_context.weight_dividor * dt * 8.0;
            let e = if -vn < rest_speed { 0.0 } else { object.get_elasticity() };
            v.0 -= (1.0 + e) * vn * nx;
            v.1 -= (1.0 + e) * vn * ny;
        }
        object.set_velocity(v);
    }
}

pub fn calculate_rotation<T: Object>(windy_context: &WindyContext, objects: &mut Vec<T>, pull_angle: f64, dt: Duration) {
    for object in objects {
        let mut rotation_set_values: Vec<(f64, Coordinate2D)> = vec![];
        let com = object.get_center_of_mass();
        let collisions = object.get_collision_info();
        for collision in collisions.iter() {
            for point in &collision.collision {
                let push_angle = Line2D(point.point, com).angle();
                rotation_set_values.push(((push_angle - pull_angle) / object.get_mass() / windy_context.torque_divisor * dt.as_secs_f64(), point.point));
            }
        }
        for v in rotation_set_values {
            object.rotate(v.0, v.1);
        }
    }
}

pub fn calculate_force<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects {
        let object_velocity = object.get_velocity();
        let object_position = object.get_position();
        object.set_position(Coordinate2D { x: object_position.x + (object_velocity.0 * dt.as_secs_f64()), y: object_position.y + (object_velocity.1 * dt.as_secs_f64()) });
        //object.rotate(object.get_torque() * dt.as_secs_f64());
    }
}
