use crate::{
    component::{ray::Ray, world::HitRecord},
    math::Vector3,
};

use super::Material;

pub struct Lambert {
    albedo: Vector3,
}

impl Lambert {
    pub fn new(albedo: Vector3) -> Lambert {
        Lambert { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let mut scatter_direction = record.normal + Vector3::random_unit();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        let scattered = Ray::new(record.point, scatter_direction);
        Some((scattered, self.albedo.clone()))
    }
}
