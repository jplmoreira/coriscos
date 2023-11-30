use crate::{component::ray::Ray, geometry::HitRecord, math::Vector3};

use super::{Material, MaterialRef, ScatterResult};

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> MaterialRef {
        Box::new(Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        })
    }
}

impl Material for Metal {
    fn scatter(&self, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = record.direction.normalize().reflect(&record.normal);

        let scattered = Ray::new(record.point, reflected + self.fuzz * Vector3::random_unit());
        Some(ScatterResult {
            t: record.t,
            ray: scattered,
            attenuation: self.albedo.clone(),
        })
    }
}
