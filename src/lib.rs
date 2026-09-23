use std::{any::Any, collections::HashMap, hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}, sync::Arc, vec};
use twodimensional::shape::Mesh2D;

pub mod gravity;
pub mod twodimensional;

const PI: f64 = 3.141592653589793;

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

    pub fn has_implemented(other: &twodimensional::shape::Shape2D) -> bool {
        match other {
            twodimensional::shape::Shape2D::Coordinate2D(_) => {
                true
            }
            twodimensional::shape::Shape2D::Line2D(_) => {
                false
            }
            twodimensional::shape::Shape2D::Circle2D(_) => {
                true
            }
            twodimensional::shape::Shape2D::Mesh2D(_) => {
                true
            }
            twodimensional::shape::Shape2D::Rectangle2D(_) => {
                true
            }
        }
    }
    
    pub fn collide(&self, other: &twodimensional::shape::Shape2D) -> bool {
        match other {
            twodimensional::shape::Shape2D::Coordinate2D(coordinate) => {
                if self.x == coordinate.x && self.y == coordinate.y {
                    return true;
                }
                
                false
            }
            twodimensional::shape::Shape2D::Line2D(_) => {
                true
            }
            twodimensional::shape::Shape2D::Circle2D(circle) => {
                let dist_x = self.x - circle.0.x;
                let dist_y = self.y - circle.0.y;
                let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();
                if distance < circle.1 {
                    return true;
                }

                false
            }
            twodimensional::shape::Shape2D::Mesh2D(_) => {
                true
            }
            twodimensional::shape::Shape2D::Rectangle2D(rect) => {
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

impl AddAssign for Coordinate2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;
    }
}

impl Sub for Coordinate2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Coordinate2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}

impl Mul for Coordinate2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl MulAssign for Coordinate2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}

impl Div for Coordinate2D {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl DivAssign for Coordinate2D {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x; self.y /= rhs.y;
    }
}

impl DivAssign<usize> for Coordinate2D {
    fn div_assign(&mut self, rhs: usize) {
        self.x /= rhs as f64; self.y /= rhs as f64;
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

pub trait Object {
    fn get_position(&self) -> &Coordinate2D;
    fn set_position(&mut self, position: Coordinate2D);
    fn get_mass(&self) -> f64;
    fn get_force(&self) -> f32;
    fn rotate(&self, angle: f64);
}

pub trait CollisionObject: Any {
    fn colliding(&self, other: &dyn Any) -> bool;
}

pub struct CollisionHash<T: CollisionObject> {
    hash: HashMap<Arc<T>, (Arc<T>, bool)>,
}

impl<T: CollisionObject + 'static + Eq + Hash> CollisionHash<T> {
    pub fn new(objects: Vec<Arc<T>>) -> Self {
        let mut hash: HashMap<Arc<T>, (Arc<T>, bool)> = HashMap::new();
        
        for object in &objects {
            for object_2 in &objects {
                if !std::ptr::eq(object, object_2) {
                    hash.insert(object.clone(), (object_2.clone(), object.colliding(object_2)));
                }
            }
        }

        Self { hash }
    }

    pub fn contains(&self, k: &T) -> bool {
        if self.hash.contains_key(k) {
            return true;
        }

        false
    }
}

pub struct WindyContext {
    weight_dividor: f64,
}

impl WindyContext {
    pub fn default() -> Self {
        Self { weight_dividor: 10.0 }
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
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: 0.1, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: 0.2, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.6, });

        assert!(!line.collide(&line_2.into()))
    }
   
    #[test]
    fn line2d_collision_2() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.2, y: 0.7}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_3() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.6, y: 0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_4() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.3, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_5() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.6, y: -0.2}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(line.collide(&line_2.into()))
    }
    
    #[test]
    fn line2d_collision_6() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.6, y: 0.0}, Coordinate2D{ x: 0.3, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }

    #[test]
    fn line2d_collision_7() { 
        let line: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.4, y: 0.1}, Coordinate2D{ x: 0.2, y: 0.5, });
        let line_2: twodimensional::shape::Line2D = twodimensional::shape::Line2D(Coordinate2D{ x: -0.5, y: 0.0}, Coordinate2D{ x: -0.2, y: 0.7, });

        assert!(!line.collide(&line_2.into()))
    }
}
