use std::{ops::{Add, Sub}, vec};
pub mod gravity;
pub mod shape;

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

    pub fn has_implemented(other: &shape::Shape2D) -> bool {
        match other {
            shape::Shape2D::Coordinate2D(_) => {
                true
            }
            shape::Shape2D::Line2D(_) => {
                false
            }
            shape::Shape2D::Circle2D(_) => {
                true
            }
            shape::Shape2D::Mesh2D(_) => {
                true
            }
            shape::Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
    
    pub fn collide(&self, other: &shape::Shape2D) -> bool {
        match other {
            shape::Shape2D::Coordinate2D(coordinate) => {
                if self.x == coordinate.x && self.y == coordinate.y {
                    return true;
                }
                
                false
            }
            shape::Shape2D::Line2D(_) => {
                true
            }
            shape::Shape2D::Circle2D(circle) => {
                let dist_x = self.x - circle.0.x;
                let dist_y = self.y - circle.0.y;
                let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
                if distance < circle.1 {
                    return true;
                }

                false
            }
            shape::Shape2D::Mesh2D(_) => {
                true
            }
            shape::Shape2D::Rectangle2D(rect) => {
                if rect.0.x > self.x
                && rect.0.x + rect.1.x < self.x
                && rect.0.y > self.y
                && rect.0.y + rect.1.y < self.y {
                    return true;
                }

                false
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
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: 0.1, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: 0.2, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.6, });

        assert!(!line.collide(&line_2.into()))
    }
   
    #[test]
    fn line2d_collision_2() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.2, y: 0.7}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_3() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.6, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_4() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_5() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(line.collide(&line_2.into()))
    }
    
    #[test]
    fn line2d_collision_6() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.6, y: 0.0}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_7() { 
        let line: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: shape::Line2D = shape::Line2D(Coordinate2D{ x: -0.5, y: 0.0}, Coordinate2D{ x: -0.2, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }
}
