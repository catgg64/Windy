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

pub fn calculate_collision_pushback<T: Object>(
    windy_context: &WindyContext,
    objects: &mut Vec<T>,
    dt: Duration,
) {
    let dt = dt.as_secs_f64();

    for object in objects.iter_mut() {        
        let edges: Vec<(f64, f64, f64, f64)> = {
            let info = object.get_collision_info();
            if info.is_empty() { continue; }
            info[0].collision.iter()
                .map(|c| (c.edge.0.x, c.edge.0.y, c.edge.1.x, c.edge.1.y))
                .collect()
        };
        if edges.is_empty() { continue; }
        if edges.len() == 2 {
            let gravity = object.get_mass() / windy_context.weight_dividor;
            let rest_speed = gravity * dt * 8.0;
            let elasticity = object.get_elasticity();

            let mut v = object.get_velocity();
            let mut pos = object.get_position().clone();
            let v0 = v; // velocity at the start of the frame
            let prev_x = pos.x - v0.0 * dt;
            let prev_y = pos.y - v0.1 * dt;

            // 1. Build the list of UNIQUE surface normals (oriented toward the object).
            let mut normals: Vec<(f64, f64)> = Vec::new();
            for (x0, y0, x1, y1) in edges {
                let (dx, dy) = (x1 - x0, y1 - y0);
                let len = (dx * dx + dy * dy).sqrt();
                if len < 1e-9 { continue; }

                let (mut nx, mut ny) = (-dy / len, dx / len);

                let mut side = (prev_x - x0) * nx + (prev_y - y0) * ny;
                if side.abs() < 1e-6 {
                    side = (pos.x - x0) * nx + (pos.y - y0) * ny;
                }
                if side.abs() < 1e-6 {
                    side = if ny > 0.0 { -1.0 } else { 1.0 }; // y grows downward
                }
                if side < 0.0 { nx = -nx; ny = -ny; }

                // Same surface already handled (two points on one edge)? Skip it.
                if normals.iter().any(|&(ax, ay)| ax * nx + ay * ny > 0.999) {
                    continue;
                }
                normals.push((nx, ny));
            }

            // 2. Push out and bounce once per unique surface, judged by the
            //    start-of-frame velocity so processing order doesn't matter.
            for &(nx, ny) in &normals {
                let vn0 = v0.0 * nx + v0.1 * ny;
                if vn0 >= 0.0 { continue; }

                let resting = -vn0 < rest_speed;
                let depth = -vn0 * dt * if resting { 0.8 } else { 1.0 };
                pos.x += nx * depth;
                pos.y += ny * depth;

                let vn = v.0 * nx + v.1 * ny;
                if vn < 0.0 {
                    let e = if resting { 0.0 } else { elasticity };
                    v.0 -= (1.0 + e) * vn * nx;
                    v.1 -= (1.0 + e) * vn * ny;
                }
            }

            // 3. Settle: remove any leftover velocity into any surface (no bounce).
            //    This is what stops corner jitter.
            for _ in 0..4 {
                for &(nx, ny) in &normals {
                    let vn = v.0 * nx + v.1 * ny;
                    if vn < 0.0 {
                        v.0 -= vn * nx;
                        v.1 -= vn * ny;
                    }
                }
            }

            object.set_position(pos);
            object.set_velocity(v);
        }
        else if edges.len() == 1 {
            let object_position = object.get_position();
            let object_velocity = object.get_velocity();
            object.set_position(Coordinate2D { x: object_position.x - ((object_velocity.0 * dt) * 1.01), y: object_position.y - ((object_velocity.1 * dt) * 1.01) });

            object.set_velocity(
            (-object.get_velocity().0 * object.get_elasticity(),
            -object.get_velocity().1 * object.get_elasticity(),
            ));
        }
    }
}

pub fn calculate_force<T: Object>(objects: &mut Vec<T>, dt: Duration) {
    for object in objects {
        let object_velocity = object.get_velocity();
        let object_position = object.get_position();
        object.set_position(Coordinate2D { x: object_position.x + (object_velocity.0 * dt.as_secs_f64()), y: object_position.y + (object_velocity.1 * dt.as_secs_f64()) });
    }
}
