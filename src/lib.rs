use core::panic;
use std::{ops::{Add, Sub}, vec};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
}

impl Coordinate2D {
    pub fn into_number_array(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }

    pub fn into_byte_array(&self) -> [u8; 16] {
        [self.x.to_be_bytes(), self.y.to_be_bytes()].concat().try_into().unwrap()
    }

    pub fn subtract_and_divide(&self, rhs: Self) -> f64 {
        let c = Coordinate2D{ x: self.x - rhs.x, y: self.y - rhs.y };
        c.x / c.y
    }

    pub fn has_implemented(other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(_) => {
                true
            }
            Shape2D::Line2D(_) => {
                false
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
            Shape2D::Triangle2D(_) => {
                true
            } 
        }
    }
    
    pub fn collide(&self, other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(coordinate) => {
                if self.x == coordinate.x && self.y == coordinate.y {
                    return true;
                }
                
                false
            }
            Shape2D::Line2D(_) => {
                true
            }
            Shape2D::Circle2D(circle) => {
                let dist_x = self.x - circle.0.x;
                let dist_y = self.y - circle.0.y;
                let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
                if distance < circle.1 {
                    return true;
                }

                false
            }
            Shape2D::Mesh2D(_) => {
                true
            }
            Shape2D::Rectangle2D(rect) => {
                if rect.0.x > self.x
                && rect.0.x + rect.1.x < self.x
                && rect.0.y > self.y
                && rect.0.y + rect.1.y < self.y {
                    return true;
                }

                false
            }
            Shape2D::Triangle2D(_) => {
                true
            } 
        }
    }
}

impl Add for Coordinate2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Coordinate2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Line2D(pub Coordinate2D, pub Coordinate2D);

impl Line2D {
    pub fn subtract_and_divide(&self) -> f64 {
        let c = Coordinate2D{ x: self.0.x - self.1.x, y: self.0.y - self.1.y};
        //println!("{:?}", c.y / c.x);
        //println!("{}, {}", d1, d2);
        c.x / c.y
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
            Shape2D::Triangle2D(_) => {
                false
            } 
        }
    }
    
    pub fn collide(&self, other: &Shape2D) -> bool {
        match other {
            Shape2D::Coordinate2D(coordinate) => {
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
                if ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) >= 0.0 && ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1)) <= 1.0 {
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

                let on_segment = self.collide(&Shape2D::Coordinate2D(Coordinate2D{ x: closest_x, y: closest_y }));

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
                if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), Coordinate2D { x: rect.0.x + rect.1.x, y: rect.0.y }))) {
                    return true;
                }
                else if self.collide(&Shape2D::Line2D(Line2D(rect.0.clone(), Coordinate2D { x: rect.0.x + rect.1.x, y: rect.0.y + rect.1.y }))) {
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
pub struct Triangle2D(Coordinate2D, Coordinate2D, Coordinate2D);

impl From<(Coordinate2D, Coordinate2D, Coordinate2D)> for Triangle2D {
    fn from(value: (Coordinate2D, Coordinate2D, Coordinate2D)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Rectangle2D(Coordinate2D, Coordinate2D);

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
            Shape2D::Triangle2D(_) => {
                false
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Circle2D(Coordinate2D, f64);

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
            Shape2D::Triangle2D(_) => {
                false
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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mesh2D {
    pub coordinates: Vec<Coordinate2D>
}

impl Mesh2D {
    pub fn colliding(&self, rhs: &Shape2D) -> bool {
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

            _ => {panic!("not implemented")}
        }
    }
}

pub enum Shape2D {
    Coordinate2D(Coordinate2D),
    Line2D(Line2D),
    Triangle2D(Triangle2D),
    Mesh2D(Mesh2D),
    Rectangle2D(Rectangle2D),
    Circle2D(Circle2D),
}

fn point_to_point_collision(v: Coordinate2D, o: Coordinate2D) -> bool {
    if v.x == o.x && v.y == o.y {
        return true;
    }

    false
}

#[repr(C)]
#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coordinate3D {
    pub fn into_byte_array(&self) -> [u8; 24] {
        [self.x.to_be_bytes(), self.y.to_be_bytes(), self.z.to_be_bytes()].concat().try_into().unwrap()
    }
}

impl Add for Coordinate3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Triangle3D(Coordinate3D, Coordinate3D, Coordinate3D);

impl From<(Coordinate3D, Coordinate3D, Coordinate3D)> for Triangle3D {
    fn from(value: (Coordinate3D, Coordinate3D, Coordinate3D)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn add_two_coordinate_2d() {
        let c1 = Coordinate2D {
            x: 0.5,
            y: 0.5
        };
        
        let c2 = Coordinate2D {
            x: 1.5,
            y: 0.5
        };

        assert_eq!(c1 + c2, Coordinate2D { x: 2.0, y: 1.0 })
    }

    #[test]
    fn add_two_coordinate_3d() {
        let c1 = Coordinate3D {
            x: 0.5,
            y: 0.5,
            z: 1.5,
        };
        
        let c2 = Coordinate3D {
            x: 1.5,
            y: 0.5,
            z: 1.5
        };

        assert_eq!(c1 + c2, Coordinate3D { x: 2.0, y: 1.0, z: 3.0 })
    }

    #[test]
    fn line2d_collision_1() {
        let line: Line2D = Line2D(Coordinate2D{ x: 0.1, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: 0.2, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.6, });

        assert!(!line.collide(&line_2.into()))
    }
   
    #[test]
    fn line2d_collision_2() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.2, y: 0.7}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_3() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_4() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_5() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(line.collide(&line_2.into()))
    }
    
    #[test]
    fn line2d_collision_6() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: 0.0}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_7() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.5, y: 0.0}, Coordinate2D{ x: -0.2, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }
}
