use crate::{
    component::{ray::Ray, world::HitRecord},
    math::Vector3,
};

use super::Material;

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let reflected = ray.direction().normalize().reflect(&record.normal);

        let scattered = Ray::new(record.point, reflected + self.fuzz * Vector3::random_unit());
        Some((scattered, self.albedo.clone()))
    }
}
