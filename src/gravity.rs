use std::{f64::consts::PI, println, time::Duration};

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
        object.set_velocity(object.get_velocity() + object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64());
        //object.set_direction((object.get_direction() + pull_angle) / 2.0);
        object.set_position(object.get_position().clone() - Coordinate2D{ x: object.get_velocity() * object.get_direction().cos() * dt.as_secs_f64(), y: object.get_velocity() * object.get_direction().sin() * dt.as_secs_f64() });
    }
}

pub fn calculate_collision_pushback<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects.iter_mut() {
        let mut changed_direction = false;
        let mut new_direction = 0.0;

        let collisions = object.get_collision_info();
        if !collisions.is_empty() {
            for collision in collisions {
                for point in &collision.collision {
                    let object_direction = object.get_direction();
                    let movement = Coordinate2D {
                        x: object_direction.cos(),
                        y: object_direction.sin(),
                    };
                    
                    let mut normal = Coordinate2D {
                        x: (point.angle + PI / 2.0).cos(),
                        y: (point.angle + PI / 2.0).sin(),
                    };
                    
                    let dot =
                        movement.x * normal.x +
                        movement.y * normal.y;
                    
                    if dot > 0.0 {
                        normal.x = -normal.x;
                        normal.y = -normal.y;
                    }

                    let push_angle = normal.y.atan2(normal.x);
                    
                    new_direction = push_angle;
                    changed_direction = true;
                }
            }
        }

//        println!("{}", new_direction);
        if changed_direction {
            object.set_direction(new_direction);
        }
    }
}