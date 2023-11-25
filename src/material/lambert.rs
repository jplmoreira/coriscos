use crate::{component::ray::Ray, geometry::HitRecord, math::Vector3};

use super::{Material, MaterialRef, ScatterResult};

pub struct Lambert {
    albedo: Vector3,
}

impl Lambert {
    pub fn new(albedo: Vector3) -> MaterialRef {
        Box::new(Self { albedo })
    }
}

impl Material for Lambert {
    fn scatter(&self, record: HitRecord) -> ScatterResult {
        let mut scatter_direction = record.normal + Vector3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        ScatterResult {
            t: record.t,
            ray: scattered,
            attenuation: self.albedo.clone(),
        }
    }
}
