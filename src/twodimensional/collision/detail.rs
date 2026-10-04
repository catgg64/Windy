use crate::{Coordinate2D, twodimensional::{shape::{Line2D, Mesh2D}}};

#[derive(Debug)]
pub struct DetailedPoint {
    pub point: Coordinate2D,
    pub edge: Line2D,
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
    let u_a = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    let u_b = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));

    if u_a >= 0.0 && u_a <= 1.0 && u_b <= 1.0 && u_b >= 0.0 {
        return Some(Coordinate2D{ x: x1 + (u_a * (x2 - x1)), y: y1 + (u_a * (y2 - y1)) })
    }

    None
}

pub fn detailed_collision_mesh_line(a: &Mesh2D, b: &Line2D) -> Option<(Coordinate2D, Line2D, Coordinate2D)> {
    for i in 0..a.coordinates.len() {
        let vc = &(a.coordinates[i] + a.origin);
        let vn = &(a.coordinates[(i + 1) % a.coordinates.len()] + a.origin);
        let line = Line2D(*vc, *vn);

        let collision = detailed_collision_line_line(b, &Line2D(vn.clone(), vc.clone()));
        if let Some(collision) = collision {
            return Some((collision, line.clone(), line.1 - collision ));
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
            collision.collision.push(DetailedPoint { point: is_colliding.0, edge: is_colliding.1, penetration: 0.0 });
        }
    }

    // if collision_mesh_coordinate(a, &(b.coordinates[0] + b.origin)) {
    //     collision.collision.push(DetailedPoint { point: b.coordinates[0] + b.origin, edge: Line2D(Coordinate2D { x: 0.0, y: 0.0 }, Coordinate2D { x: 10.0, y: 10.0 }), penetration: 0.0 });
    // }

    collision
}
