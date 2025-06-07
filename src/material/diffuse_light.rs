use crate::math::Vector3;

use super::Material;

pub struct DiffuseLight {
    color: Vector3,
}

impl DiffuseLight {
    pub fn new(color: Vector3) -> Self {
        Self { color }
    }
}

impl Material for DiffuseLight {
    fn emit(&self) -> Vector3 {
        self.color.clone()
    }
}
