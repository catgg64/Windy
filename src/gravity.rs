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
        object.set_force(object.get_force() + object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64());
        object.set_direction((object.get_direction() + pull_angle) / 2.0);
        object.set_position(object.get_position().clone() - Coordinate2D{ x: object.get_force() * object.get_direction().cos() * dt.as_secs_f64(), y: object.get_force() * object.get_direction().sin() * dt.as_secs_f64() });
    }
}

pub fn calculate_collision_pushback<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects.iter_mut() {
        let mut new_direction = 0.0;

        let collisions = object.get_collision_info();
        if !collisions.is_empty() {
            for collision in collisions {
                for point in &collision.collision {
                    let object_direction = object.get_direction();
                    //if object_direction > point.angle && object_direction < point.angle * PI {
                        let value = ((point.angle + PI / 2.0) % PI).abs();
                        println!("{}", value);
                        if new_direction.is_none() {
                            new_direction = Some(value);
                        }
                        else {
                            if let Some(new_new_direction) = new_direction {
                                new_direction = Some((value + new_new_direction) / 2.0);
                            }
                        }
                    }
                    if object_direction < point.angle && object_direction > point.angle * PI {
                        println!("here");
                        if new_direction.is_none() {
                            new_direction = Some(((point.angle + PI / 2.0) % PI));
                        }
                        else {
                            if let Some(new_new_direction) = new_direction {
                                new_direction = Some((((point.angle + PI / 2.0) % PI) + new_new_direction) / 2.0);
                            }
                        }
                    }
                }
            }
        }

        if let Some(direction) = new_direction {
            object.set_direction(direction);
        }
    }
}