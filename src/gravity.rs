use crate::{Coordinate2D, WindyContext, shape::Mesh2D};

#[derive(Debug)]
pub struct Object {
    pub position: Coordinate2D,
    pub weight: f32,
    pub force: f32,
    pub mesh: Mesh2D,
}

pub struct OrbitObject {
    pub position: Coordinate2D,
    pub weight: f32,
}

impl OrbitObject {
    pub fn new(objects: Vec<Object>) -> Self {
        let mut weight = 0.0;
        let mut pos = Coordinate2D{ x: 0.0, y: 0.0 };
        for object in &objects {
            weight += object.weight;
            pos += object.position.clone();
        }
        pos /= objects.len();

        Self { position: pos, weight }
    }
}

// pub enum Space {
//     Objects(Vec<Object>),
//     OrbitObjects(Vec<OrbitObject>),
// }

// fn calculate_gravitational_pull(windy_context: WindyContext, space: Space, position: Coordinate2D) -> (f32, f32) {
//     match space {
//         Space::Objects(objects) => {
//             let angle = 0.0;
//             let strengh = 0.0;
//             for object in objects {
//                 let object_position = object.position;
                
//             } 

//             (angle, strengh)
//         }
//     }
// }