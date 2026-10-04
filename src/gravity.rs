use std::{println, time::Duration};

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
        object.set_velocity((object_velocity.0 + 0.0, object_velocity.1 + object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64()));
        //object.set_velocity(object.get_velocity() + object.get_mass() / windy_context.weight_dividor * dt.as_secs_f64());
        //object.set_direction((object.get_direction() + pull_angle) / 2.0);
        //object.set_position(object.get_position().clone() - Coordinate2D{ x: object.get_velocity() * object.get_direction().cos() * dt.as_secs_f64(), y: object.get_velocity() * object.get_direction().sin() * dt.as_secs_f64() });
        //object.add_force(());
    }
}

pub fn calculate_collision_pushback<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects.iter_mut() {
        let mut new_velocity = object.get_velocity();

        if !object.get_collision_info()[0].collision.is_empty() {
            let object_position = object.get_position();
            let object_velocity = object.get_velocity();
            object.set_position(Coordinate2D { x: object_position.x - ((object_velocity.0 * dt.as_secs_f64()) * 1.001), y: object_position.y - ((object_velocity.1 * dt.as_secs_f64()) * 1.001) });
        }

        // if !object.get_collision_info()[0].collision.is_empty() {
        //     let object_position = object.get_position();
        //     let object_velocity = object.get_velocity();
        //     object.set_position(Coordinate2D { x: object_position.x - (object_velocity.0 * dt.as_secs_f64()), y: object_position.y - (object_velocity.1 * dt.as_secs_f64()) });
        // }
        
        
        let collisions = object.get_collision_info();
        if !collisions[0].collision.is_empty() {
            if collisions[0].collision.len() == 2 {
                let point = &collisions[0].collision[0];
                let object_velocity = object.get_velocity();
                    let movement = Coordinate2D {
                        x: object_velocity.0,
                        y: object_velocity.1,
                    };
                    
                    let dx = point.edge.1.x - point.edge.0.x;
                    let dy = point.edge.1.y - point.edge.0.y;

                    let mut normal = Coordinate2D {
                        x: -dy,
                        y: dx,
                    };

                    let length = (normal.x * normal.x + normal.y * normal.y).sqrt();

                    normal.x /= length;
                    normal.y /= length;
                    
                    let dot =
                        movement.x * normal.x +
                        movement.y * normal.y;
                    
                    
                    if dot > 0.0 {
                        normal.x = -normal.x;
                        normal.y = -normal.y;
                    }

                    let vn =
                        new_velocity.0 * normal.x +
                        new_velocity.1 * normal.y;

                    if vn < 0.0 {
                        let e = object.get_elasticity();

                        
                        new_velocity.0 -= (1.0 + e) * vn * normal.x;
                        new_velocity.1 -= (1.0 + e) * vn * normal.y;
                    
                    }


                //println!("dot: {}", dot);
                // let normal_velocity = 
                //     object_velocity.0 * normal.x +
                //     object_velocity.1 * normal.y;
                
                // new_force = (
                //     object_velocity.0
                //         - normal.x * normal_velocity * (1.0 + object.get_elasticity()),

                //     object_velocity.1
                //         - normal.y * normal_velocity * (1.0 + object.get_elasticity()),
                // );


                //let push_angle = normal.y.atan2(normal.x);
                
                //println!("{:?}", point.angle);
                //new_force = (normal.x * object_velocity.0 / object.get_elasticity(), normal.y * object_velocity.1 / object.get_elasticity());
                //changed_direction = true;
            }
            else if collisions[0].collision.len() == 1 {
                // let object_velocity = object.get_velocity();
                
                // let subtract_value = match (object_velocity.0 > 0.0, object_velocity.1 > 0.0) {
                //     (true, false) => {
                //         Coordinate2D{ x: -0.01, y: -0.01 }
                //     }
                //     (true, true) => {
                //         Coordinate2D{ x: -0.01, y: 0.01 }
                //     }
                //     (false, false) => {
                //         Coordinate2D{ x: 0.01, y: -0.01 }
                //     }
                //     (false, true) => {
                //         Coordinate2D{ x: 0.01, y: 0.01 }
                //     }
                // };
                //object.set_position(*object.get_position() - subtract_value);
                new_velocity = (
                    -new_velocity.0 / (1.0 + object.get_elasticity()),
                    -new_velocity.1 / (1.0 + object.get_elasticity()),
                );
                println!("{:?}", new_velocity);
            }
        }

        //if should_set {
        //}

//        println!("{}", new_direction);
        object.set_velocity(new_velocity);
    }
}

pub fn calculate_force<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects {
        let object_velocity = object.get_velocity();
        let object_position = object.get_position();
        object.set_position(Coordinate2D { x: object_position.x + (object_velocity.0 * dt.as_secs_f64()), y: object_position.y + (object_velocity.1 * dt.as_secs_f64()) });
    }
}
