use std::{sync::Arc, vec};

use crate::{Coordinate2D, WindyContext, twodimensional::shape::{Line2D, Mesh2D}, Object};

// #[derive(Debug)]
// pub struct Object {
//     pub position: Coordinate2D,
//     pub mass: f64,
//     pub force: f32,
//     pub mesh: Mesh2D,
// }

// pub struct OrbitObject<T: Object> {
//     pub position: Coordinate2D,
//     pub weight: f64,
// }

// impl<T: Object> OrbitObject<T> {
//     pub fn new(objects: Vec<T>) -> Self {
//         let mut weight = 0.0;
//         let mut pos = Coordinate2D{ x: 0.0, y: 0.0 };
//         for object in &objects {
//             weight += object.mass;
//             pos += object.position.clone();
//         }
//         pos /= objects.len();

//         Self { position: pos, weight: weight }
//     }
// }

// pub enum Space {
//     Objects(Vec<Object>),
//     OrbitObjects(Vec<OrbitObject>),
// }

pub fn calculate_gravitational_pull<T: Object>(windy_context: WindyContext, objects: Vec<T>, position: &Coordinate2D) -> (f64, f64) {
    let mut angle = 0.0;
    let mut strengh = 0.0;
    for object in &objects {
        let object_position = &object.get_pos();
        angle += Line2D(position.clone(), object_position.clone()).angle();
        strengh += object.get_mass();
    }
    angle /= objects.len() as f64;
 
    (angle, strengh / windy_context.weight_dividor)
}

pub fn calculate_physics_step<T: Object>(windy_context: WindyContext, mut objects: Vec<T>, static_objects: Vec<T>, pull_angle: f64) {
    let mut change_queue: Vec<(&T, Coordinate2D)> = vec![];
    
    for object in &objects {
        let mut is_colliding = false;
        for static_object in &static_objects {
            if static_object.colliding(object.get_mesh()) {
                is_colliding = true;
            }
        }
        for object2 in &objects {
            if object2.colliding(object.get_mesh()) {
                is_colliding = true;
            }
        }

        if is_colliding {
            let object_pos = object.get_position();
            change_queue.push((&object, Coordinate2D { x: object_pos.x - 5.0, y: object_pos.y - 5.0 }));
            //object.set_position(&Coordinate2D { x: object_pos.x - 5.0, y: object_pos.y - 5.0 });
        }
    }

    for mut object_index in 0..objects.len() {
        let object = objects[object_index];
        for change in change_queue {
            if object == change.0 {
                object.set_position(object.get_position() - Coordinate2D{ x: object.get_mass() / windy_context.weight_dividor * pull_angle.cos(), y: object.get_mass() / windy_context.weight_dividor * pull_angle.cos() });
            }
        }
    }
    //for mut object in objects {
    //    object.position -= Coordinate2D{ x: object.mass / windy_context.weight_dividor * pull_angle.cos(), y: object.mass / windy_context.weight_dividor * pull_angle.cos() }
    //}
}