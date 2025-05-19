use crate::{
    component::{hit::HitRecord, ray::Ray},
    math::Vector3,
};

use super::{Material, ScatterResult};

pub struct Lambert {
    albedo: Vector3,
}

impl Lambert {
    pub fn new(albedo: Vector3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, record: HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = &record.normal + Vector3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal.clone();
        }

        let scattered = Ray::new(record.buf_idx, record.point.clone(), scatter_direction);
        Some(ScatterResult {
            _t: record.t,
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
