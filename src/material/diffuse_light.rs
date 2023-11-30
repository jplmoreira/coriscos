use crate::{geometry::HitRecord, math::Vector3};

use super::{Material, MaterialRef, ScatterResult};

pub struct DiffuseLight {
    color: Vector3,
}

impl DiffuseLight {
    pub fn new(color: Vector3) -> MaterialRef {
        Box::new(Self { color })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _record: &HitRecord) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, _record: &HitRecord) -> Vector3 {
        return self.color;
    }
}
