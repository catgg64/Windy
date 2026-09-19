use std::sync::Arc;
use crate::Coordinate2D;

pub fn collide(lhs: &Shape2D, rhs: &Shape2D) -> bool {
    if match lhs {
        Shape2D::Coordinate2D(lhs) => {
            if Coordinate2D::has_implemented(rhs) {
                return lhs.collide(rhs);
            }

            false
        }
        
        Shape2D::Circle2D(lhs) => {
            if Circle2D::has_implemented(rhs) {
                return lhs.collide(rhs);
            }

            false
        }

        Shape2D::Line2D(lhs) => {
            if Line2D::has_implemented(rhs) {
                return lhs.collide(rhs);
            }

            false
        }
        Shape2D::Rectangle2D(lhs) => {
            if Rectangle2D::has_implemented(rhs) {
                return lhs.collide(rhs);
            }

            false
        }
        Shape2D::Mesh2D(lhs) => {
            if Mesh2D::has_implemented(rhs) {
                return lhs.collide(rhs);
            }

            false
        }
    } { return true; }
    else {
        match rhs {
            Shape2D::Coordinate2D(rhs) => {
                if Coordinate2D::has_implemented(lhs) {
                    return rhs.collide(lhs);
                }

                false
            }
            
            Shape2D::Circle2D(rhs) => {
                if Circle2D::has_implemented(lhs) {
                    return rhs.collide(lhs);
                }

                false
            }

            Shape2D::Line2D(rhs) => {
                if Line2D::has_implemented(lhs) {
                    return rhs.collide(lhs);
                }

                false
            }
            Shape2D::Rectangle2D(rhs) => {
                if Rectangle2D::has_implemented(lhs) {
                    return rhs.collide(lhs);
                }

                false
            }
            Shape2D::Mesh2D(left) => {
                if Mesh2D::has_implemented(lhs) {
                    return left.collide(lhs);
                }

                false
            }
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Line2D(pub crate::Coordinate2D, pub crate::Coordinate2D);

impl Line2D {
    pub fn subtract_and_divide(&self) -> f64 {
        let c = crate::Coordinate2D{ x: self.0.x - self.1.x, y: self.0.y - self.1.y};
        //println!("{:?}", c.y / c.x);
        //println!("{}, {}", d1, d2);
        c.x / c.y
    }

    pub fn angle(&self) -> f64 {
        self.1.y - self.0.y.atan2(self.1.x - self.0.x)
    }

    pub fn distance(&self) -> f64 {
        let dist_x = self.0.x - self.1.x;
        let dist_y = self.0.y - self.1.y;

        ((dist_x * dist_x) + (dist_y * dist_y)).sqrt()
    }
    
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

impl Into<Shape2D> for Line2D {
    fn into(self) -> Shape2D {
        Shape2D::Line2D(self)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Triangle2D(pub crate::Coordinate2D, pub crate::Coordinate2D, pub crate::Coordinate2D);

impl From<(crate::Coordinate2D, crate::Coordinate2D, crate::Coordinate2D)> for Triangle2D {
    fn from(value: (crate::Coordinate2D, crate::Coordinate2D, crate::Coordinate2D)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rectangle2D(pub crate::Coordinate2D, pub crate::Coordinate2D);

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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Circle2D(pub crate::Coordinate2D, pub f64);

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

#[repr(C)]
#[derive(Debug)]
pub struct Mesh2D {
    pub coordinates: Arc<Vec<crate::Coordinate2D>>
}

impl Mesh2D {
    pub fn new(coordinates: Vec<crate::Coordinate2D>) -> Self {
        Self { coordinates: Arc::new(coordinates) }
    }

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

impl Clone for Mesh2D {
    fn clone(&self) -> Self {
        Self { coordinates: self.coordinates.clone() }
    }
}

impl Into<Shape2D> for Mesh2D {
    fn into(self) -> Shape2D {
        Shape2D::Mesh2D(self)
    }
}

pub enum Shape2D {
    Coordinate2D(crate::Coordinate2D),
    Line2D(Line2D),
    Mesh2D(Mesh2D),
    Rectangle2D(Rectangle2D),
    Circle2D(Circle2D),
}