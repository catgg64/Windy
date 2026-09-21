use std::sync::Arc;

use crate::{Coordinate2D, WindyContext, shape::{Circle2D, Line2D, Mesh2D}};

// #[derive(Debug)]
// pub struct Object {
//     pub position: Coordinate2D,
//     pub mass: f64,
//     pub force: f32,
//     pub mesh: Mesh2D,
// }

pub trait Object {
    fn get_pos(&self) -> Coordinate2D;
    fn get_mass(&self) -> f64;
    fn get_force(&self) -> f32;
    fn rotate(&self, angle: f64);
}

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

pub fn calculate_gravitational_pull<T: Object>(windy_context: WindyContext, space: Vec<T>, position: &Coordinate2D) -> (f64, f64) {
    
}

pub fn calculate_physics_step(windy_context: WindyContext, objects: Vec<Object>, static_objects: Vec<Object>, pull_angle: f64) {
    for mut object in objects {
        object.position -= Coordinate2D{ x: object.mass / windy_context.weight_dividor * pull_angle.cos(), y: object.mass / windy_context.weight_dividor * pull_angle.cos() }
    }
}