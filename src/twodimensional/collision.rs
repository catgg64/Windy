use crate::{CollisionObject, Coordinate2D, math, twodimensional::shape::{Circle2D, Line2D, Mesh2D, Rectangle2D, Shape2D}};

pub fn collision_coordinate_coordinate(a: &Coordinate2D, b: &Coordinate2D) -> bool {
    if a.x == b.x && a.y == b.y {
        return true;
    }
    
    false
}

pub fn collision_coordinate_circle(a: &Coordinate2D, b: &Circle2D) -> bool {
    let dist_x = a.x - b.0.x;
    let dist_y = a.y - b.0.y;
    let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
    if distance < b.1 {
        return true;
    }

    false
}

pub fn collision_coordinate_rect(a: &Coordinate2D, b: &Rectangle2D) -> bool {
    if b.0.x > a.x
    && b.0.x + b.1.x < a.x
    && b.0.y > a.y
    && b.0.y + b.1.y < a.y {
        return true;
    }

    false
}

pub fn collision_line_line(a: &Line2D, b: &Line2D) -> bool {
    let x1 = a.0.x;
    let x2 = a.1.x;
    let x3 = b.0.x;
    let x4 = b.1.x;
    let y1 = a.0.y;
    let y2 = a.1.y;
    let y3 = b.0.y;
    let y4 = b.1.y;
    if ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) >= 0.0 && ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) <= 1.0 && ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) >= 0.0 {
        return true
    }

    false       
}

pub fn collision_line_coordinate(a: &Line2D, b: &Coordinate2D) -> bool {
    let line_len = math::dist(a.0.clone().into(), a.1.clone().into());
    let d1 = math::dist(b.clone().into(), a.0.clone().into());
    let d2 = math::dist(b.clone().into(), a.1.clone().into());

    if d1 + d2 >= line_len {
        return true;
    }

    false
}

pub fn collision_line_circle(a: &Line2D, b: &Circle2D) -> bool {
    let inside_0 = collision_coordinate_circle(&a.0, b);
    let inside_1 = collision_coordinate_circle(&a.1, b);

    if inside_0 || inside_1 { return true; }

    let dist_x = a.0.x - a.1.x;
    let dist_y = a.0.y - a.1.y;
    let len = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
    let dot = ( ((b.0.x-a.0.x)*(a.1.x-a.0.x)) + ((b.0.y-a.0.y)*(a.1.y-a.0.y)) ) / f64::powf(len,2.0);
    let closest_x = a.0.x + (dot * (a.1.x - a.0.x));
    let closest_y = a.0.y + (dot * (a.1.y - a.0.y));

    let on_segment = collision_line_coordinate(a, &Coordinate2D{ x: closest_x, y: closest_y });

    if !on_segment { return false; }

    let dist_x = closest_x - b.0.x;
    let dist_y = closest_y - b.0.y;
    let len = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

    if len <= b.1 {
        return true;
    }

    false
}

pub fn collision_line_rect(a: &Line2D, b: &Rectangle2D) -> bool {
    if collision_line_line(a, &Line2D(b.0.clone(), crate::Coordinate2D { x: b.0.x + b.1.x, y: b.0.y })) {
        return true;
    }
    else if collision_line_line(a, &Line2D(b.0.clone(), crate::Coordinate2D { x: b.0.x + b.1.x, y: b.0.y + b.1.y })) {
        return true;
    }
    else if collision_line_line(a, &Line2D(b.0.clone(), crate::Coordinate2D { x: b.0.x, y: b.0.y})) {
        return true;
    }
    else if collision_line_line(a, &Line2D(b.0.clone(), crate::Coordinate2D { x: b.0.x, y: b.0.y + b.1.y })) {
        return true;
    }

    false
}

pub fn collision_rect_rect(a: &Rectangle2D, b: &Rectangle2D) -> bool {
    if b.0.x + b.1.x >= a.0.x 
    && b.0.x <= a.0.x + a.1.x
    && b.0.y + b.1.y >= a.0.y 
    && b.0.y <= a.0.y + a.1.y {
        return true;
    }

    false
}

pub fn collision_circle_circle(a: &Circle2D, b: &Circle2D) -> bool {
    let dist_x = a.0.x - b.0.x;
    let dist_y = a.0.y - b.0.y;

    if ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() <= a.1 + b.1 {
        return true;
    }
    
    false
}

pub fn collision_circle_rect(a: &Circle2D, b: &Rectangle2D) -> bool {
    let mut test_x = a.0.x;
    let mut test_y = a.0.y;

    if a.0.x < b.0.x { test_x = b.0.x }
    else if a.0.x > b.0.x + b.1.x { test_x = b.0.x + b.1.x }

    if a.0.y < b.0.y { test_y = b.0.y }
    else if a.0.y > b.0.y + b.1.y { test_y = b.0.y + b.1.y }

    let dist_x = a.0.x - test_x;
    let dist_y = a.0.y - test_y;

    if ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() <= a.1 {
        return true;
    }

    false
}

pub fn collision_mesh_coordinate(a: &Mesh2D, b: &Coordinate2D) -> bool {
    let px = b.x;
    let py = b.y;
    let mut colliding = false;
    for vc in 0..a.coordinates.len() {
        if vc < a.coordinates.len() {
            let vn = &a.coordinates[vc + 1];
            let vc = &a.coordinates[vc];
            if ((vc.y > py) != (vn.y > py)) && (px < (vn.x-vc.x) * (py-vc.y) / (vn.y-vc.y) + vc.x) {
                colliding = !colliding
            }
        }
        
        
    }

    colliding
}

pub fn collision_mesh_line(a: &Mesh2D, b: &Line2D) -> bool {
    for vc in 0..a.coordinates.len() {
        if vc < a.coordinates.len() {
            let vn = &a.coordinates[vc + 1];
            let vc = &a.coordinates[vc];
            
            if collision_line_line(b, &Line2D(vn.clone(), vc.clone())) {
                return true;
            }
        }
    }

    false
}

pub fn collision_mesh_circle(a: &Mesh2D, b: &Circle2D) -> bool {
    for vc in 0..a.coordinates.len() {
        if vc < a.coordinates.len() {
            let vn = &a.coordinates[vc + 1];
            let vc = &a.coordinates[vc];

            if collision_line_circle(&Line2D(vn.clone(), vc.clone()), b) {
                return true;
            }
        }
    }

    if collision_mesh_coordinate(a, &b.0) {
        return true;
    }

    false
}

pub fn collision_mesh_rect(a: &Mesh2D, b: &Rectangle2D) -> bool {
    for vc in 0..a.coordinates.len() {
        if vc < a.coordinates.len() {
            let vn = &a.coordinates[vc + 1];
            let vc = &a.coordinates[vc];
            
            if collision_line_rect(&Line2D(vn.clone(), vc.clone()), b) {
                return true;
            }
        }
    }

    if collision_mesh_coordinate(a, &b.0) {
        return true;
    }

    false
}

pub fn collision_mesh_mesh(a: &Mesh2D, b: &Mesh2D) -> bool {
    for vc in 0..a.coordinates.len() {
        if vc < a.coordinates.len() {
            let vn = &a.coordinates[vc + 1];
            let vc = &a.coordinates[vc];
            
            if collision_mesh_line(b, &Line2D(vn.clone(), vc.clone())) {
                return true;
            }
        }
    }

    if collision_mesh_coordinate(a, &b.coordinates[0]) {
        return true;
    }

    false
}

impl CollisionObject for Line2D {
    fn colliding(&self, other: &dyn std::any::Any) -> bool {
        if let Some(coordinate) = other.downcast_ref::<Coordinate2D>() {
            return collision_line_coordinate(self, coordinate);
        }
        if let Some(line) = other.downcast_ref::<Line2D>() {
            return collision_line_line(self, line);
        }
        if let Some(circle) = other.downcast_ref::<Circle2D>() {
            return collision_line_circle(self, circle);
        }
        if let Some(rect) = other.downcast_ref::<Rectangle2D>() {
            return collision_line_rect(self, rect);
        }
        if let Some(mesh) = other.downcast_ref::<Mesh2D>() {
            return collision_mesh_line(mesh, self);
        }

        false
    }
}

impl CollisionObject for Rectangle2D {
    fn colliding(&self, other: &dyn std::any::Any) -> bool {
        if let Some(coordinate) = other.downcast_ref::<Coordinate2D>() {
            return collision_coordinate_rect(coordinate, self);
        }
        if let Some(line) = other.downcast_ref::<Line2D>() {
            return collision_line_rect(line, self);
        }
        if let Some(circle) = other.downcast_ref::<Circle2D>() {
            return collision_circle_rect(circle, self);
        }
        if let Some(rect) = other.downcast_ref::<Rectangle2D>() {
            return collision_rect_rect(self, rect);
        }
        if let Some(mesh) = other.downcast_ref::<Mesh2D>() {
            return collision_mesh_rect(mesh, self);
        }

        false
    }
}

impl Into<Shape2D> for Rectangle2D {
    fn into(self) -> Shape2D {
        Shape2D::Rectangle2D(self)
    }
}

impl CollisionObject for Circle2D {
    fn colliding(&self, other: &dyn std::any::Any) -> bool {
        if let Some(coordinate) = other.downcast_ref::<Coordinate2D>() {
            return collision_coordinate_circle(coordinate, self);
        }
        if let Some(line) = other.downcast_ref::<Line2D>() {
            return collision_line_circle(line, self);
        }
        if let Some(circle) = other.downcast_ref::<Circle2D>() {
            return collision_circle_circle(circle, self);
        }
        if let Some(rect) = other.downcast_ref::<Rectangle2D>() {
            return collision_circle_rect(self, rect);
        }
        if let Some(mesh) = other.downcast_ref::<Mesh2D>() {
            return collision_mesh_circle(mesh, self);
        }

        false
    }
}

impl Into<Shape2D> for Circle2D {
    fn into(self) -> Shape2D {
        Shape2D::Circle2D(self)
    }
}

impl Mesh2D {
    pub fn has_implemented(other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(_) => {
                true
            }
            Shape2D::Line2D(_) => {
                true
            }
            Shape2D::Circle2D(_) => {
                true
            }
            Shape2D::Mesh2D(_) => {
                true
            }
            Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
}
    
impl CollisionObject for Mesh2D {
    fn colliding(&self, other: &dyn std::any::Any) -> bool {
        if let Some(coordinate) = other.downcast_ref::<Coordinate2D>() {
            return collision_mesh_coordinate(self, coordinate);
        }
        if let Some(line) = other.downcast_ref::<Line2D>() {
            return collision_mesh_line(self, line);
        }
        if let Some(circle) = other.downcast_ref::<Circle2D>() {
            return collision_mesh_circle(self, circle);
        }
        if let Some(rect) = other.downcast_ref::<Rectangle2D>() {
            return collision_mesh_rect(self, rect);
        }
        if let Some(mesh) = other.downcast_ref::<Mesh2D>() {
            return collision_mesh_mesh(self, mesh);
        }

        false
    }
}


impl Into<Shape2D> for Mesh2D {
    fn into(self) -> Shape2D {
        Shape2D::Mesh2D(self)
    }
}