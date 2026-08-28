#[repr(C)]
#[derive(Debug, Clone)]
pub struct Coordinate2D {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Coordinate3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
