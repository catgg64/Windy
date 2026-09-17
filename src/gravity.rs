use crate::shape::Mesh2D;

#[derive(Debug)]
pub struct Object {
    pub weight: f32,
    pub force: f32,
    pub mesh: Mesh2D,
}

pub struct PlanetObject {
    pub objects: Vec<Object>,
}