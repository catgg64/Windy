use crate::{Coordinate2D, math, twodimensional::shape::{Circle2D, Line2D, Mesh2D, Rectangle2D, Shape2D}};

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

    let on_segment = a.collide(&Shape2D::Coordinate2D(crate::Coordinate2D{ x: closest_x, y: closest_y }));

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
            
            if b.collide(&Shape2D::Line2D(Line2D(vn.clone(), vc.clone()))) {
                return true;
            }
        }
    }

    if collision_mesh_coordinate(a, &b.coordinates[0]) {
        return true;
    }

    false
}

impl Line2D {
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
                false
            }
            Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
    
    pub fn collide(&self, other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(_) => {
                false
            }

            Shape2D::Line2D(line) => {
                let x1 = self.0.x;
                let x2 = self.1.x;
                let x3 = line.0.x;
                let x4 = line.1.x;
                let y1 = self.0.y;
                let y2 = self.1.y;
                let y3 = line.0.y;
                let y4 = line.1.y;
                if ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) >= 0.0 && ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) <= 1.0 && ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) >= 0.0 {
                    return true
                }

                false       
            }

            Shape2D::Circle2D(circle) => {
                let inside_0 = self.0.collide(other);
                let inside_1 = self.1.collide(other);

                if inside_0 || inside_1 { return true; }

                let dist_x = self.0.x - self.1.x;
                let dist_y = self.0.y - self.1.y;
                let len = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
                let dot = ( ((circle.0.x-self.0.x)*(self.1.x-self.0.x)) + ((circle.0.y-self.0.y)*(self.1.y-self.0.y)) ) / f64::powf(len,2.0);
                let closest_x = self.0.x + (dot * (self.1.x - self.0.x));
                let closest_y = self.0.y + (dot * (self.1.y - self.0.y));

                let on_segment = self.collide(&Shape2D::Coordinate2D(crate::Coordinate2D{ x: closest_x, y: closest_y }));

                if !on_segment { return false; }

                let dist_x = closest_x - circle.0.x;
                let dist_y = closest_y - circle.0.y;
                let len = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

                if len <= circle.1 {
                    return true;
                }

                false
            }

            Shape2D::Rectangle2D(rect) => {
                if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), crate::Coordinate2D { x: rect.0.x + rect.1.x, y: rect.0.y }))) {
                    return true;
                }
                else if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), crate::Coordinate2D { x: rect.0.x + rect.1.x, y: rect.0.y + rect.1.y }))) {
                    return true;
                }
                else if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), crate::Coordinate2D { x: rect.0.x, y: rect.0.y}))) {
                    return true;
                }
                else if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), crate::Coordinate2D { x: rect.0.x, y: rect.0.y + rect.1.y }))) {
                    return true;
                }

                false
            }

            _ => { false }
        }
    }
}

impl Rectangle2D {
    pub fn has_implemented(other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(_) => {
                false
            }
            Shape2D::Line2D(_) => {
                false
            }
            Shape2D::Circle2D(_) => {
                false
            }
            Shape2D::Mesh2D(_) => {
                false
            }
            Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
    
    pub fn collide(&self, other: &Shape2D) -> bool {
        match other {
            Shape2D::Rectangle2D(rect) => {
                if rect.0.x + rect.1.x >= self.0.x 
                && rect.0.x <= self.0.x + self.1.x
                && rect.0.y + rect.1.y >= self.0.y 
                && rect.0.y <= self.0.y + self.1.y {
                    return true;
                }

                false
            }

            _ => { false }
        }
    }
}

impl Into<Shape2D> for Rectangle2D {
    fn into(self) -> Shape2D {
        Shape2D::Rectangle2D(self)
    }
}


impl Circle2D {
    pub fn has_implemented(other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(_) => {
                false
            }
            Shape2D::Line2D(_) => {
                false
            }
            Shape2D::Circle2D(_) => {
                true
            }
            Shape2D::Mesh2D(_) => {
                false
            }
            Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
    
    pub fn collide(&self, other: &Shape2D) -> bool {
        match other {
            Shape2D::Circle2D(circle) => {
                let dist_x = self.0.x - circle.0.x;
                let dist_y = self.0.y - circle.0.y;

                if ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() <= self.1 + circle.1 {
                    return true;
                }
                
                false
            }

            Shape2D::Rectangle2D(rect) => {
                let mut test_x = self.0.x;
                let mut test_y = self.0.y;

                if self.0.x < rect.0.x { test_x = rect.0.x }
                else if self.0.x > rect.0.x + rect.1.x { test_x = rect.0.x + rect.1.x }

                if self.0.y < rect.0.y { test_y = rect.0.y }
                else if self.0.y > rect.0.y + rect.1.y { test_y = rect.0.y + rect.1.y }

                let dist_x = self.0.x - test_x;
                let dist_y = self.0.y - test_y;

                if ((dist_x * dist_x) + (dist_y * dist_y)).sqrt() <= self.1 {
                    return true;
                }

                false
            }

            _ => { false }
        }
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

    pub fn collide(&self, rhs: &Shape2D) -> bool {
        match rhs {
            Shape2D::Coordinate2D(coordinate) => {
                let px = coordinate.x;
                let py = coordinate.y;
                let mut colliding = false;
                for vc in 0..self.coordinates.len() {
                    if vc < self.coordinates.len() {
                        let vn = &self.coordinates[vc + 1];
                        let vc = &self.coordinates[vc];
                        if ((vc.y > py) != (vn.y > py)) && (px < (vn.x-vc.x) * (py-vc.y) / (vn.y-vc.y) + vc.x) {
                            colliding = !colliding
                        }
                    }
                    
                    
                }

                colliding
            }

            Shape2D::Circle2D(circle) => {
                for vc in 0..self.coordinates.len() {
                    if vc < self.coordinates.len() {
                        let vn = &self.coordinates[vc + 1];
                        let vc = &self.coordinates[vc];
                
                        if circle.collide(&Shape2D::Line2D(Line2D(vn.clone(), vc.clone()))) {
                            return true;
                        }
                    }
                }

                if circle.0.collide(&Shape2D::Mesh2D(self.clone())) {
                    return true;
                }
                
                false
            }

            Shape2D::Rectangle2D(rect) => {
                for vc in 0..self.coordinates.len() {
                    if vc < self.coordinates.len() {
                        let vn = &self.coordinates[vc + 1];
                        let vc = &self.coordinates[vc];
                        
                        if rect.collide(&Shape2D::Line2D(Line2D(vn.clone(), vc.clone()))) {
                            return true;
                        }
                    }
                }

                if rect.0.collide(&Shape2D::Mesh2D(self.clone())) {
                    return true;
                }

                false
            }

            Shape2D::Line2D(line) => {
                for vc in 0..self.coordinates.len() {
                    if vc < self.coordinates.len() {
                        let vn = &self.coordinates[vc + 1];
                        let vc = &self.coordinates[vc];
                        
                        if line.collide(&Shape2D::Line2D(Line2D(vn.clone(), vc.clone()))) {
                            return true;
                        }
                    }
                }

                false
            }

            Shape2D::Mesh2D(mesh) => {
                for vc in 0..self.coordinates.len() {
                    if vc < self.coordinates.len() {
                        let vn = &self.coordinates[vc + 1];
                        let vc = &self.coordinates[vc];
                        
                        if mesh.collide(&Shape2D::Line2D(Line2D(vn.clone(), vc.clone()))) {
                            return true;
                        }
                    }
                }

                if mesh.coordinates[0].collide(&Shape2D::Mesh2D(self.clone())) {
                    return true;
                }

                false
            }
        }
    }
}

impl Into<Shape2D> for Mesh2D {
    fn into(self) -> Shape2D {
        Shape2D::Mesh2D(self)
    }
}