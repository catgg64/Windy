use std::{sync::Arc, vec};
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
        let angle = (self.1.y - self.0.y).atan2(self.1.x - self.0.x);

        angle
    }

    pub fn distance(&self) -> f64 {
        let dist_x = self.0.x - self.1.x;
        let dist_y = self.0.y - self.1.y;

        ((dist_x * dist_x) + (dist_y * dist_y)).sqrt()
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Circle2D(pub crate::Coordinate2D, pub f64);

#[repr(C)]
#[derive(Debug)]
pub struct Mesh2D {
    pub coordinates: Arc<Vec<crate::Coordinate2D>>
}

impl Mesh2D {
    pub fn new(coordinates: Vec<crate::Coordinate2D>) -> Self {
        Self { coordinates: Arc::new(coordinates) }
    }

    pub fn rotate(&mut self, angle: f64, origin: Coordinate2D) {
        let mut new_mesh: Vec<crate::Coordinate2D> = vec![];

        for coordinate in self.coordinates.iter() {
            let set_coordinate = coordinate.clone() - origin.clone();
            set_coordinate.
            new_mesh.push(coordinate.clone() - origin.clone());
        }
    }
}

impl Clone for Mesh2D {
    fn clone(&self) -> Self {
        Self { coordinates: self.coordinates.clone() }
    }
}

pub enum Shape2D {
    Coordinate2D(crate::Coordinate2D),
    Line2D(Line2D),
    Mesh2D(Mesh2D),
    Rectangle2D(Rectangle2D),
    Circle2D(Circle2D),
}