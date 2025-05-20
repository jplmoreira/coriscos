use crate::{
    component::{hit::HitRecord, ray::Ray},
    math::Vector3,
};

use super::{Material, ScatterResult};

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, record: HitRecord) -> Option<ScatterResult> {
        let reflected = record.direction.normal().reflect(&record.normal);

        let scattered = Ray::new(
            record.point.clone(),
            reflected + Vector3::random_unit() * self.fuzz,
        );
        Some(ScatterResult {
            _t: record.t,
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
