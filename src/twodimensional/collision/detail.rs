use std::f64::consts::PI;

use crate::{Coordinate2D, twodimensional::{collision::{collision_mesh_coordinate}, shape::{Line2D, Mesh2D}}};

#[derive(Debug)]
pub struct DetailedPoint {
    pub point: Coordinate2D,
    pub angle: f64,
    pub penetration: f64,
}

#[derive(Debug)]
pub struct DetailedCollision {
    pub collision: Vec<DetailedPoint>,
}

pub fn detailed_collision_line_line(a: &Line2D, b: &Line2D) -> Option<Coordinate2D> {
    let x1 = a.0.x;
    let x2 = a.1.x;
    let x3 = b.0.x;
    let x4 = b.1.x;
    let y1 = a.0.y;
    let y2 = a.1.y;
    let y3 = b.0.y;
    let y4 = b.1.y;
    let uA = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let uB = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));

    if uA >= 0.0 && uA <= 1.0 && uB <= 1.0 && uB >= 0.0 {
        return Some(Coordinate2D{ x: x1 + (uA * (x2 - x1)), y: y1 + (uA * (y2 - y1)) })
    }

    None
}

pub fn detailed_collision_mesh_line(a: &Mesh2D, b: &Line2D) -> Option<Coordinate2D> {
    for i in 0..a.coordinates.len() {
        let vc = &(a.coordinates[i] + a.origin);
        let vn = &(a.coordinates[(i + 1) % a.coordinates.len()] + a.origin);

        let collision = detailed_collision_line_line(b, &Line2D(vn.clone(), vc.clone()));
        if collision.is_some() {
            return collision;
        }
    }

    None
}

pub fn detailed_collision_mesh_mesh(a: &Mesh2D, b: &Mesh2D) -> DetailedCollision {
    let mut collision = DetailedCollision{ collision: vec![]};
    
    for i in 0..a.coordinates.len() {
        let vc = &(a.coordinates[i] + a.origin);
        let vn = &(a.coordinates[(i + 1) % a.coordinates.len()] + a.origin);

        let is_colliding = detailed_collision_mesh_line(b, &Line2D(vc.clone(), vn.clone())); 
        if let Some(is_colliding) = is_colliding {
            collision.collision.push(DetailedPoint { point: is_colliding, angle: Line2D(*vc, *vn).angle(), penetration: 0.0 });
        }
    }

    if collision_mesh_coordinate(a, &(b.coordinates[0] + b.origin)) {
        collision.collision.push(DetailedPoint { point: b.coordinates[0] + b.origin, angle: 0.0, penetration: 0.0 });
    }

    collision
}
