use std::{ops::Add, vec};

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
}

impl Add for Coordinate2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Line2D(pub Coordinate2D, pub Coordinate2D);

impl Line2D {
    fn collide(&self, rhs: Self) -> bool {
        true
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

        assert!(!line.collide(line_2))
    }
    
    #[test]
    fn line2d_collision_2() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: 0.2, y: -0.7}, Coordinate2D{ x: 0.3, y: 0.6, });

        assert!(!line.collide(line_2))
    }
    
    #[test]
    fn line2d_collision_3() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.2, y: 0.7}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(line_2))
    }

    #[test]
    fn line2d_collision_4() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(line_2))
    }

    #[test]
    fn line2d_collision_5() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(!line.collide(line_2))
    }

    #[test]
    fn line2d_collision_6() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(line.collide(line_2))
    }
    
    #[test]
    fn line2d_collision_7() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.6, y: 0.0}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(!line.collide(line_2))
    }

    #[test]
    fn line2d_collision_8() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.5, y: 0.0}, Coordinate2D{ x: -0.2, y: 0.7, });

        assert!(!line.collide(line_2))
    }

    #[test]
    fn line2d_collision_9() { 
        let line: Line2D = Line2D(Coordinate2D{ x: -0.5, y: 0.0}, Coordinate2D{ x: -0.2, y: 0.7, });
        let line_2: Line2D = Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });

        assert!(!line.collide(line_2))
    }
}
